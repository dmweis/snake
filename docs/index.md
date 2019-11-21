---
title: Snake Game
---

<canvas id="snake_canvas" width="640" height="640" style="cursor: auto;"></canvas>
<script>
    var Module = {};
    var __cargo_web = {};
    Object.defineProperty( Module, 'canvas', {
        get: function() {
            if( __cargo_web.canvas ) {
                return __cargo_web.canvas;
            }
            __cargo_web.canvas = document.document.getElementById("snake_canvas");
            return canvas;
        }
    });
</script>
<script src="snake.js"></script>
