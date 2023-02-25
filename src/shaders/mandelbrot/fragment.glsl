#version 330 core
out vec4 FragColor;

in vec2 TexCoords;
in vec3 Position;
in vec2 TransPos;

// Found at https://github.com/hughsk/glsl-hsv2rgb
vec3 hsv2rgb(vec3 c) {
  vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
  vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
  return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    int max_iter = 100;
    int iter = 0;

    // Use the translated position as the complex number input.
    // Store the original 'c value' for use later.
    float cReal = TransPos.x;
    float cImag = TransPos.y;

    // Will hold the most recent iteration of the series
    float zReal = cReal;
    float zImag = cImag;

    // create a rough approximation on if the set is stable
    for (iter = 0; iter < max_iter; iter++) {
        float zr2 = zReal * zReal;
        float zi2 = zImag * zImag;

        float newReal = zr2 - zi2;
        float newImag = 2 * zReal * zImag;

        zReal = newReal + cReal;
        zImag = newImag + cImag;

        // if above threshold it is not considered stable
        if (zr2 + zi2 > 80) {
            break;
        }
    }

    if (max_iter == iter) {
        // just show black if it was found to be stable
        FragColor = vec4(vec3(0.0), 1.0);
    } else {
        // if unstable give it a color
        float brightness = float(iter) / float(max_iter);
        brightness = sqrt(brightness);
        FragColor = vec4(hsv2rgb(vec3(brightness, 1.0, 1.0)), 1.0);
    }
}
