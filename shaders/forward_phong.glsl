program vert {
    #version 150

    uniform mat4 modelTransform;
    uniform mat4 normalTransform;
    uniform mat4 modelViewTransform;
    uniform mat4 modelViewProjection;
    uniform mat4 viewTransform;
    uniform mat4 projectionTransform;

    in vec4 vertexPosition;
    in vec3 vertexNormal;

    out vec4 worldPosition;
    out vec4 viewPosition;
    out vec3 viewNormal;

    void main(void) {
        worldPosition = modelTransform * vertexPosition;
        viewPosition = modelViewTransform * vertexPosition;
        viewNormal = normalize(mat3(normalTransform) * vertexNormal);
        gl_Position = modelViewProjection * vertexPosition;
    }
}

program frag {
    #version 150

    uniform vec4 cameraPosition;
    uniform vec4 lightPosition;
    uniform vec4 globalAmbient;
    uniform mat4 viewTransform;
    uniform mat4 modelViewTransform;

    in vec4 worldPosition;
    in vec4 viewPosition;
    in vec3 viewNormal;

    out vec4 fragmentColor;

    void main(void) {
        // STUFF THAT NEEDS TO BECOME UNIFORMS
        vec4 surfaceDiffuse = vec4(1.0, 0.0, 1.0, 1.0);
        vec4 lightColor = vec4(1.0, 1.0, 1.0, 1.0);
        vec4 surfaceSpecular = vec4(1.0, 1.0, 1.0, 1.0);
        float surfaceShininess = 3.0;
        float light_radius = 5.0;

        // Calculate phong illumination.
        vec4 ambient = vec4(0.0, 0.0, 0.0, 1.0);
        vec4 diffuse = vec4(0.0, 0.0, 0.0, 1.0);
        vec4 specular = vec4(0.0, 0.0, 0.0, 1.0);

        ambient = globalAmbient * surfaceDiffuse;

        vec3 light_offset = (lightPosition - viewPosition).xyz;
        float dist = length(light_offset);

        vec3 N = normalize(viewNormal);
        vec3 L = normalize(light_offset);
        vec3 V = normalize(-viewPosition.xyz);

        float LdotN = dot(L, N);
        float attenuation = 1.0 / pow((dist / light_radius) + 1.0, 2.0);

        diffuse += surfaceDiffuse * lightColor * max(LdotN, 1.0e-6) * attenuation;

        if (LdotN > 1e-6) {
            vec3 R = normalize(reflect(-L, N));
            float RdotV = clamp(dot(R, V), 0.0, 1.0);
            specular = surfaceSpecular * lightColor * pow(RdotV, surfaceShininess) * attenuation;
        }

        fragmentColor = ambient + diffuse + specular;
    }
}
