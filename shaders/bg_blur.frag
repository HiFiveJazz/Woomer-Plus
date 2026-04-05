varying vec2 uv;

uniform sampler2D Texture;
uniform vec2 texel_size;
uniform float blur_strength;

void main() {
    vec2 stepv = texel_size * blur_strength;
    vec4 sum = vec4(0.0);

    sum += texture2D(Texture, uv + stepv * vec2(-2.0, -2.0)) * 0.025;
    sum += texture2D(Texture, uv + stepv * vec2(-1.0, -2.0)) * 0.035;
    sum += texture2D(Texture, uv + stepv * vec2( 0.0, -2.0)) * 0.040;
    sum += texture2D(Texture, uv + stepv * vec2( 1.0, -2.0)) * 0.035;
    sum += texture2D(Texture, uv + stepv * vec2( 2.0, -2.0)) * 0.025;

    sum += texture2D(Texture, uv + stepv * vec2(-2.0, -1.0)) * 0.035;
    sum += texture2D(Texture, uv + stepv * vec2(-1.0, -1.0)) * 0.050;
    sum += texture2D(Texture, uv + stepv * vec2( 0.0, -1.0)) * 0.060;
    sum += texture2D(Texture, uv + stepv * vec2( 1.0, -1.0)) * 0.050;
    sum += texture2D(Texture, uv + stepv * vec2( 2.0, -1.0)) * 0.035;

    sum += texture2D(Texture, uv + stepv * vec2(-2.0,  0.0)) * 0.040;
    sum += texture2D(Texture, uv + stepv * vec2(-1.0,  0.0)) * 0.060;
    sum += texture2D(Texture, uv + stepv * vec2( 0.0,  0.0)) * 0.080;
    sum += texture2D(Texture, uv + stepv * vec2( 1.0,  0.0)) * 0.060;
    sum += texture2D(Texture, uv + stepv * vec2( 2.0,  0.0)) * 0.040;

    sum += texture2D(Texture, uv + stepv * vec2(-2.0,  1.0)) * 0.035;
    sum += texture2D(Texture, uv + stepv * vec2(-1.0,  1.0)) * 0.050;
    sum += texture2D(Texture, uv + stepv * vec2( 0.0,  1.0)) * 0.060;
    sum += texture2D(Texture, uv + stepv * vec2( 1.0,  1.0)) * 0.050;
    sum += texture2D(Texture, uv + stepv * vec2( 2.0,  1.0)) * 0.035;

    sum += texture2D(Texture, uv + stepv * vec2(-2.0,  2.0)) * 0.025;
    sum += texture2D(Texture, uv + stepv * vec2(-1.0,  2.0)) * 0.035;
    sum += texture2D(Texture, uv + stepv * vec2( 0.0,  2.0)) * 0.040;
    sum += texture2D(Texture, uv + stepv * vec2( 1.0,  2.0)) * 0.035;
    sum += texture2D(Texture, uv + stepv * vec2( 2.0,  2.0)) * 0.025;

    gl_FragColor = vec4(sum.rgb * 0.55, 1.0);
}
