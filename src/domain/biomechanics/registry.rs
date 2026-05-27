use ultraviolet::f64x4;

/// An SOA (Structure-of-Arrays) container for all rigid body components.
/// Optimized for SIMD processing and cache-line locality.
#[derive(Debug, Clone, Default)]
pub struct BodyRegistry {
    // Position
    pub pos_x: Vec<f64>,
    pub pos_y: Vec<f64>,
    pub pos_z: Vec<f64>,
    
    // Previous Position (for XPBD velocity update)
    pub prev_pos_x: Vec<f64>,
    pub prev_pos_y: Vec<f64>,
    pub prev_pos_z: Vec<f64>,

    // Velocity
    pub vel_x: Vec<f64>,
    pub vel_y: Vec<f64>,
    pub vel_z: Vec<f64>,

    // External Forces
    pub force_x: Vec<f64>,
    pub force_y: Vec<f64>,
    pub force_z: Vec<f64>,

    // Physical Properties
    pub inv_mass: Vec<f64>,
}

impl BodyRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new body to the registry and returns its index.
    pub fn add_body(&mut self, x: f64, y: f64, z: f64, mass: f64) -> usize {
        let idx = self.inv_mass.len();
        let inv_m = if mass > 0.0 { 1.0 / mass } else { 0.0 };

        self.pos_x.push(x);
        self.pos_y.push(y);
        self.pos_z.push(z);

        self.prev_pos_x.push(x);
        self.prev_pos_y.push(y);
        self.prev_pos_z.push(z);

        self.vel_x.push(0.0);
        self.vel_y.push(0.0);
        self.vel_z.push(0.0);

        self.force_x.push(0.0);
        self.force_y.push(0.0);
        self.force_z.push(0.0);

        self.inv_mass.push(inv_m);

        idx
    }

    /// Executes the prediction pass using SIMD acceleration.
    /// x* = x + v*dt + a*dt^2
    pub fn predict_simd(&mut self, dt: f64, gravity_y: f64) {
        let dt_simd = f64x4::splat(dt);
        let dt2_simd = f64x4::splat(dt * dt);
        let gy_simd = f64x4::splat(gravity_y);

        let len = self.len();
        let chunks = len / 4;

        for i in 0..chunks {
            let offset = i * 4;

            // Load positions
            let mut px = f64x4::new([self.pos_x[offset], self.pos_x[offset+1], self.pos_x[offset+2], self.pos_x[offset+3]]);
            let mut py = f64x4::new([self.pos_y[offset], self.pos_y[offset+1], self.pos_y[offset+2], self.pos_y[offset+3]]);
            let mut pz = f64x4::new([self.pos_z[offset], self.pos_z[offset+1], self.pos_z[offset+2], self.pos_z[offset+3]]);

            // Save prev positions
            self.prev_pos_x[offset..offset+4].copy_from_slice(&self.pos_x[offset..offset+4]);
            self.prev_pos_y[offset..offset+4].copy_from_slice(&self.pos_y[offset..offset+4]);
            self.prev_pos_z[offset..offset+4].copy_from_slice(&self.pos_z[offset..offset+4]);

            // Load velocities
            let vx = f64x4::new([self.vel_x[offset], self.vel_x[offset+1], self.vel_x[offset+2], self.vel_x[offset+3]]);
            let vy = f64x4::new([self.vel_y[offset], self.vel_y[offset+1], self.vel_y[offset+2], self.vel_y[offset+3]]);
            let vz = f64x4::new([self.vel_z[offset], self.vel_z[offset+1], self.vel_z[offset+2], self.vel_z[offset+3]]);

            // Load forces and inverse mass
            let fx = f64x4::new([self.force_x[offset], self.force_x[offset+1], self.force_x[offset+2], self.force_x[offset+3]]);
            let fy = f64x4::new([self.force_y[offset], self.force_y[offset+1], self.force_y[offset+2], self.force_y[offset+3]]);
            let fz = f64x4::new([self.force_z[offset], self.force_z[offset+1], self.force_z[offset+2], self.force_z[offset+3]]);
            let im = f64x4::new([self.inv_mass[offset], self.inv_mass[offset+1], self.inv_mass[offset+2], self.inv_mass[offset+3]]);

            // Calculate acceleration (f * w + g)
            let ax = fx * im;
            let ay = fy * im + gy_simd;
            let az = fz * im;

            // Predict new positions
            px += vx * dt_simd + ax * dt2_simd;
            py += vy * dt_simd + ay * dt2_simd;
            pz += vz * dt_simd + az * dt2_simd;

            // Write back
            let rx: [f64; 4] = px.into();
            let ry: [f64; 4] = py.into();
            let rz: [f64; 4] = pz.into();
            
            self.pos_x[offset..offset+4].copy_from_slice(&rx);
            self.pos_y[offset..offset+4].copy_from_slice(&ry);
            self.pos_z[offset..offset+4].copy_from_slice(&rz);
        }

        // Handle remainder
        for i in (chunks * 4)..len {
            if self.inv_mass[i] > 0.0 {
                self.prev_pos_x[i] = self.pos_x[i];
                self.prev_pos_y[i] = self.pos_y[i];
                self.prev_pos_z[i] = self.pos_z[i];

                let ax = self.force_x[i] * self.inv_mass[i];
                let ay = self.force_y[i] * self.inv_mass[i] + gravity_y;
                let az = self.force_z[i] * self.inv_mass[i];

                self.pos_x[i] += self.vel_x[i] * dt + ax * (dt * dt);
                self.pos_y[i] += self.vel_y[i] * dt + ay * (dt * dt);
                self.pos_z[i] += self.vel_z[i] * dt + az * (dt * dt);
            }
        }
    }

    pub fn apply_correction(&mut self, idx: usize, dx: f64, dy: f64, dz: f64) {
        if self.inv_mass[idx] > 0.0 {
            self.pos_x[idx] += dx;
            self.pos_y[idx] += dy;
            self.pos_z[idx] += dz;
        }
    }

    pub fn len(&self) -> usize {
        self.inv_mass.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inv_mass.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_soa_layout() {
        let mut registry = BodyRegistry::new();
        let idx = registry.add_body(1.0, 2.0, 3.0, 10.0);
        
        assert_eq!(idx, 0);
        assert_eq!(registry.pos_x[0], 1.0);
        assert_eq!(registry.pos_y[0], 2.0);
        assert_eq!(registry.pos_z[0], 3.0);
        assert_eq!(registry.inv_mass[0], 0.1);
    }

    #[test]
    fn test_predict_simd() {
        let mut registry = BodyRegistry::new();
        for _ in 0..5 {
            registry.add_body(0.0, 10.0, 0.0, 1.0);
        }

        // Gravity -9.81, dt 0.1
        registry.predict_simd(0.1, -9.81);

        for i in 0..5 {
            // Predicted y should be 10.0 + 0*0.1 + (-9.81 * 0.01) = 9.9019
            assert!((registry.pos_y[i] - 9.9019).abs() < 1e-6);
            assert_eq!(registry.prev_pos_y[i], 10.0);
        }
    }
}
