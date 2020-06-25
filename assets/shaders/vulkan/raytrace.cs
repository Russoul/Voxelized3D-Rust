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

struct Sphere{
    vec3 center;
    float rad;
};


#define SHAPE_SPHERE 0
#define SHAPE_SPHERE_SIZE 4
#define OP_UNION 0
#define OP_INTERSECTION 1
#define OP_DIFFERENCE 2
#define EPSILON 0.00001f

float raySphereIntersect(vec3 r0, vec3 rd, vec3 s0, float sr) {
    // - r0: ray origin
    // - rd: normalized ray direction
    // - s0: sphere center
    // - sr: sphere radius
    // - Returns distance from r0 to first intersecion with sphere,
    //   or -1.0 if no intersection.
    float a = dot(rd, rd);
    vec3 s0_r0 = r0 - s0;
    float b = 2.0 * dot(rd, s0_r0);
    float c = dot(s0_r0, s0_r0) - (sr * sr);
    if (b*b - 4.0*a*c < 0.0) {
        return -1.0;
    }
    return (-b - sqrt((b*b) - 4.0*a*c))/(2.0*a);
}

vec3 sphereNormal(vec3 center, vec3 p){
    float n = sqrt((p.x - center.x)*(p.x - center.x) + (p.y - center.y)*(p.y - center.y) + (p.z - center.z)*(p.z - center.z));
    return vec3( (p.x - center.x) / n, (p.y - center.y)/n, (p.z - center.z)/n );
}




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


layout(set = 0, binding = 1, std140) uniform readonly Input{
    Cam cam;
    Ray rays[4];//tl, bl, br, tr
    uint num_triangles;
    uint num_terrain;
    uint num_shapes;
    uint num_ops;
};

layout(set = 0, binding = 2, std430) buffer readonly TrData {
    Triangle3Packed triangles[];
};

layout(set = 0, binding = 3) buffer readonly TData {
    float terrain[];
};

layout(set = 0, binding = 4) buffer readonly SData {
    uint shapes[];
};

layout(set = 0, binding = 5) buffer readonly BData {
    uint binOps[];
};


Sphere getSphere(uint terrainIndex, out uint newTerrainIndex){
    Sphere sph;
    sph.center = vec3(terrain[terrainIndex + 0], terrain[terrainIndex + 1], terrain[terrainIndex + 2]);
    sph.rad = terrain[terrainIndex + 3];
    newTerrainIndex = terrainIndex + 4;
    return sph;
}

float sdfSphere(Sphere sphere, vec3 point){
    return sqrt(dot(sphere.center - point, sphere.center - point)) - sphere.rad;
}

float sdf(vec3 point){

    uint terrainIndex = 0; uint shapesIndex = 0;
    float dist = 100;

    for(uint opsIndex = 0; opsIndex < num_ops; opsIndex++){
        if(binOps[opsIndex] == OP_UNION){
            if(shapes[shapesIndex] == SHAPE_SPHERE){
                Sphere shape1 = getSphere(terrainIndex, terrainIndex);
                dist = min(dist, sdfSphere(shape1, point));
            }
        } else if(binOps[opsIndex] == OP_DIFFERENCE){
            if(shapes[shapesIndex] == SHAPE_SPHERE){
                Sphere shape1 = getSphere(terrainIndex, terrainIndex);
                dist = max(dist, -sdfSphere(shape1, point));
            }
        }else if(binOps[opsIndex] == OP_INTERSECTION){
            if(shapes[shapesIndex] == SHAPE_SPHERE){
                Sphere shape1 = getSphere(terrainIndex, terrainIndex);
                dist = max(dist, sdfSphere(shape1, point));

            }
        }
        shapesIndex += 1;
    }


    return dist;
}

vec3 normal(vec3 point){
    float eps = 0.001;
    float sdfp0 = sdf(point);

    return normalize(vec3(sdf(vec3(point.x + eps, point.y, point.z)) - sdfp0, sdf(vec3(point.x, point.y + eps, point.z)) - sdfp0, sdf(vec3(point.x, point.y, point.z + eps)) - sdfp0));
}

#define MAX_VIEW_DIST 1000

bool raytrace(Ray ray, out float distOut, out vec3 colorOut, out vec3 normalOut){
    vec3 point = ray.start;
    float maxDist = 0.001;
    float dist = MAX_VIEW_DIST;
    int k = 0;
    while(dist > maxDist && dist <= MAX_VIEW_DIST && k < 1000){
        dist = sdf(point);
        point += ray.dir * dist;
        k++;
    }
    if(dist <= maxDist){
        colorOut = vec3(1, 0, 0);
        normalOut = normal(ray.start);
        distOut = sqrt(dot(point - ray.start,point - ray.start));
        return true;
    }
    else {
        colorOut = vec3(0, 0, 0);
        return false;
    }

}

bool raytraceShadow(Ray ray){
    float maxDist = 0.001;
    float dist = MAX_VIEW_DIST;
    int k = 0;
    vec3 point = ray.start + ray.dir * maxDist;
    while(dist > maxDist && dist <= MAX_VIEW_DIST && k < 1000){
        dist = sdf(point);
        point += ray.dir * dist;
        k++;
    }
    return dist <= maxDist;

}

Triangle3 triFromPacked(Triangle3Packed packed){
    Triangle3 tri;
    tri.p1 = vec3(packed.p1x, packed.p1y, packed.p1z);
    tri.p2 = vec3(packed.p2x, packed.p2y, packed.p2z);
    tri.p3 = vec3(packed.p3x, packed.p3y, packed.p3z);

    return tri;
}

vec4 trace(Ray ray, vec2 pos){
    float nearest_hit = MAX_VIEW_DIST;

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

    if(nearest_hit < MAX_VIEW_DIST){
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
    //vec4 color = trace(ray, pos);
    vec3 color = vec3(0, 0, 0);
    vec3 normal = vec3(0, 0, 0);
    float dist;
    bool inter = raytrace(ray, dist, color, normal);

    if(inter){
        if(dot(normal, ray.dir) > 0) normal = -normal;

        vec3 point = ray.start + ray.dir * dist;
        float distToLight = sqrt(dot(point - vec3(0, 0, 0), point - vec3(0, 0, 0)));
        vec3 toLight = (vec3(0, 0, 0) - point) / distToLight;
        float intensity = 3;

        color /= distToLight * distToLight;
        color *= clamp(dot(normal, toLight), 0, 1) * intensity;
        vec3 c, n;
        float d;
        Ray shadowRay;
        shadowRay.start = point;
        shadowRay.dir = toLight;
        if(raytraceShadow(shadowRay)) color *= 0;
    }

    imageStore(img, pix, vec4(color, 1));
}