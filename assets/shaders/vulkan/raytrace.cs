#version 450


layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

struct Cam{
    vec3 pos;
    vec3 look;
    vec3 up;
};

struct Ray{
    vec3 start;
    vec3 dir;
};

struct Vertex{
    vec3 pos;
    vec3 color;
    vec3 normal;
};

struct Triangle3Packed{
    float p1x;
    float p1y;
    float p1z;
    float p2x;
    float p2y;
    float p2z;
    float p3x;
    float p3y;
    float p3z;
};

struct Triangle3{
    vec3 p1;
    vec3 p2;
    vec3 p3;
};



#define EPSILON 0.00001f

bool intersectTriangle(Ray ray, vec3 p0, vec3 p1, vec3 p2,
out float hit, out vec3 barycentricCoord, out vec3 triangleNormal)
{
    const vec3 e0 = p1 - p0;
    const vec3 e1 = p0 - p2;
    triangleNormal = cross( e1, e0 );

    const vec3 e2 = ( 1.0 / dot( triangleNormal, ray.dir ) ) * ( p0 - ray.start );
    const vec3 i  = cross( ray.dir, e2 );

    barycentricCoord.y = dot( i, e1 );
    barycentricCoord.z = dot( i, e0 );
    barycentricCoord.x = 1.0 - (barycentricCoord.z + barycentricCoord.y);
    hit   = dot( triangleNormal, e2 );

    return  /*(hit < ray.tmax) && */ (hit > EPSILON) && all(greaterThanEqual(barycentricCoord, vec3(0.0)));
}

layout(set = 0, binding = 0, rgba8) uniform writeonly image2D img;


layout(set = 0, binding = 1) uniform readonly Input{
    Cam cam;
    Ray rays[4];//tl, bl, br, tr
    int num_triangles;
};

layout(set = 0, binding = 2, std430) buffer readonly Data {
    Triangle3Packed triangles[];
};

Triangle3 triFromPacked(Triangle3Packed packed){
    Triangle3 tri;
    tri.p1 = vec3(packed.p1x, packed.p1y, packed.p1z);
    tri.p2 = vec3(packed.p2x, packed.p2y, packed.p2z);
    tri.p3 = vec3(packed.p3x, packed.p3y, packed.p3z);

    return tri;
}

vec4 trace(Ray ray, vec2 pos){
    float nearest_hit = 1000;

    for (int i = 0; i < num_triangles; i++){
        Triangle3 tri = triFromPacked(triangles[i]);
        float hit;
        vec3 barCoord;
        vec3 normal;
        if(intersectTriangle(ray, tri.p1, tri.p2, tri.p3, hit, barCoord, normal)){
            if(hit < nearest_hit){
                nearest_hit = hit;
            }
        }
    }

    if(nearest_hit < 1000){
        return vec4(0,1,0,1);
    }else{
        return vec4(0,0,0,1);
    }

}

void main() {
    ivec2 pix = ivec2(gl_GlobalInvocationID.xy);
    ivec2 size = imageSize(img);
    if (pix.x >= size.x || pix.y >= size.y) {
        return;
    }
    vec2 pos = vec2(pix) / vec2(size.x, size.y);
    vec3 dirY1 = rays[0].dir - (rays[0].dir - rays[1].dir) * pos.y;
    vec3 dirY2 = rays[3].dir - (rays[3].dir - rays[2].dir) * pos.y;
    vec3 dir = dirY1 + (dirY2 - dirY1) * pos.x;
    Ray ray;
    ray.start = cam.pos;
    ray.dir = dir;
    vec4 color = trace(ray, pos);
    imageStore(img, pix, color);
}