#version 330 core
in vec3 position;
in vec2 tex_coords;

out vec2 TexCoords;
out vec3 Position;
out vec2 TransPos;

uniform vec2 screen_dim;
uniform float zoom;
uniform vec2 screen_pos;

void main()
{
    TexCoords = tex_coords;
    Position = position;

    // TransPos is the zoomed and translated position of the vertex. Mimics the 'zoom' effect
    TransPos = (position.xy * zoom);
    TransPos.x = TransPos.x * (screen_dim.x / screen_dim.y); // keep aspect ratio
    TransPos +=  screen_pos;

    gl_Position =  vec4(position, 1.0);
}
