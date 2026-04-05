varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 cursor_uv;
uniform vec4 spotlight_tint;
uniform float spotlight_radius_multiplier;
uniform vec2 screen_size;

const float UNIT_RADIUS_PX = 60.0;
const float EDGE_SOFTNESS_PX = 24.0;

void main() {
    vec4 texel_color = texture2D(Texture, uv);

    vec2 frag_uv = gl_FragCoord.xy / screen_size;

    vec2 delta = frag_uv - cursor_uv;
    delta.x *= screen_size.x / screen_size.y;

    float radius = (UNIT_RADIUS_PX * spotlight_radius_multiplier) / screen_size.y;
    float softness = EDGE_SOFTNESS_PX / screen_size.y;

    float dist = length(delta);
    float mask = smoothstep(radius, radius + softness, dist);

    vec4 tinted = mix(texel_color, vec4(spotlight_tint.rgb, texel_color.a), spotlight_tint.a);
    gl_FragColor = mix(texel_color, tinted, mask);
}
