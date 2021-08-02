const rust = import('./pkg');
const canvas = document.getElementById('rustCanvas');
const gl = canvas.getContext('webgl', { antialias: true });

rust.then(r => {
    if (!gl) {
        alert(' Failed to initialize WebGL');
        return;
    }

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA);

    const client = new r.Client();
    const initialTime = Date.now();
    const FPS_THROTTLE = 1000.0 / 144.0; // ms / frame
    let lastDrawTime = -1; // in ms

    function render() {
        window.requestAnimationFrame(render);
        const currentTime = Date.now();
        if (currentTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currentTime;

            // Update canvas size on resize
            if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                canvas.height = window.innerHeight;
                canvas.clientHeight = window.innerHeight;
                canvas.style.height = window.innerHeight;

                canvas.width = window.innerWidth;
                canvas.clientWidth = window.innerWidth;
                canvas.style.width = window.innerWidth;

                gl.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsedTime = currentTime - initialTime;

            // Rust update and render call
            client.update(elapsedTime, window.innerHeight, window.innerWidth);
            client.render();
        }
    }

    render();
})