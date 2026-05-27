organism ProtoCrawler {
    bone Head {
        mass = 1.2 kg;
    }
    bone Tail {
        mass = 0.8 kg;
    }
    joint Spine {
        source = Head;
        target = Tail;
    }
    muscle Retractor {
        source = Head;
        target = Tail;
        max_force = 150.0 Nm;
    }
}
