---
title: Snake
---

<canvas if="snake_canvas" width="640" height="640" style="cursor: auto;"></canvas>
<script>
    var Module = {};
    var __cargo_web = {};
    Object.defineProperty( Module, 'canvas', {
        get: function() {
            if( __cargo_web.canvas ) {
                return __cargo_web.canvas;  
            }
            var canvas = document.getElementById("snake_canvas");
            __cargo_web.canvas = canvas;
            return canvas;
        }
    });
</script>
<script src="snake.js"></script>
