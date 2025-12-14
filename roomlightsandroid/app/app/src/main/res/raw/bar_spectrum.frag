#version 300 es
precision highp float;

in vec2 fragCoord;
out vec4 fragColor;

// Bar values - left channels first (low to high), then right (high to low).
uniform vec4 bars[64];
uniform int bars_count; // Number of bars (left + right)
uniform vec3 u_resolution; // Window resolution

// Transparent
const vec4 BACKGROUND_COLOR = vec4(0.0, 0.0, 0.0, 0.0);
// Pastel red
const vec4 BAR_TOP_COLOR = vec4(1.0, 0.4, 0.4, 1.0);
// Dastel purple
const vec4 BAR_BOTTOM_COLOR = vec4(0.6, 0.4, 1.0, 1.0);

void main() {
    // Find which bar to use based on where we are on the x axis
    float x = u_resolution.x * fragCoord.x;
    int bar = int(float(bars_count) * fragCoord.x);
    int bar_4 = bar / 4;
    int bar_xyz = int(mod(float(bar), 4.0));

    // The y coordinate and bar values are the same
    float y;
    if(bar_xyz == 0) y = bars[bar_4].x;
    else if(bar_xyz == 1) y = bars[bar_4].y;
    else if(bar_xyz == 2) y = bars[bar_4].z;
    else if(bar_xyz == 3) y = bars[bar_4].w;

    // Calculate bar size
    float bar_size = u_resolution.x / float(bars_count);
    float bar_spacey = floor(bar_size * 0.1);

    // Make sure there is a thin line at bottom
    if(y * u_resolution.y < 1.0) {
        y = 1.0 / u_resolution.y;
    }

    // Draw the bar up to current height
    if(y > fragCoord.y) {
        // Space between bars
        if(bar_spacey >= 1.0 && x > float(bar + 1) * bar_size - bar_spacey) {
            fragColor = BACKGROUND_COLOR;
        } else {
            // Make color
            fragColor = mix(BAR_BOTTOM_COLOR, BAR_TOP_COLOR, fragCoord.y);
        }
    } else {
        fragColor = BACKGROUND_COLOR;
    }
}
