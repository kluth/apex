const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();
  
  page.on('console', msg => console.log('BROWSER LOG:', msg.text()));

  await page.goto('http://localhost:8000/examples/viewer.html');
  await page.waitForFunction(() => {
    const el = document.getElementById('boneCount');
    return el && parseInt(el.innerText) > 0;
  }, { timeout: 15000 });
  await page.waitForTimeout(3000);
  
  // Take screenshot from front
  await page.screenshot({ path: 'apex_human_front.png' });
  
  // Rotate camera to see from side
  await page.evaluate(() => {
    // We can't easily control OrbitControls from here without exposing it
    // But we can move the camera manually if we wanted.
  });
  
  await browser.close();
})();
