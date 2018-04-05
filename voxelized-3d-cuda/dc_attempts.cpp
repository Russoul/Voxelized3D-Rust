//
// Created by russoul on 29.03.18.
//

struct Line{
    float3 start;
    float3 end;
};

struct Cell{
    float3 intersections[12];
    float3 normals[12];
    float3 minimizer;
    int config;
};

struct Grid{
    float *densities; //
    Cell *cells;
};

__device__ __host__ bool density_change(float a, float b){
    return a >= 0 ? b < 0 : b >= 0;
}

__device__ float3 sample_intersection(Line &line){
float best = noise_func(line.start);
float3 best_point = line.start;
for (int i = 1; i <= 10; ++i) {
float3 pos = line.start + (line.end - line.start) / 10.0F * i;
float sample = noise_func(pos);
if(abs(sample) < abs(best)){
best = sample;
best_point = pos;
}
}

return best_point;
}

__global__ void kernel_fill_grid(Grid grid, float3 offset, float a){
    int i = blockIdx.x * blockDim.x + threadIdx.x;

    int x = i % blockDim.x;//TODO blockDim.x must be equal to grid size in one dim
    int y = (i / blockDim.x) % blockDim.x;
    int z = (i / blockDim.x / blockDim.x) % blockDim.x;

    grid.densities[i] = noise_func(offset + make_float3(x * a, y * a, z * a));
}

__global__ void kernel_fill_grid2(Grid grid, float3 offset, float a){//slower than the first variant
    int i = blockIdx.x * blockDim.x + threadIdx.x; //index of a core
    int k = 5461; //should be 5462

    for (int j = 0; j < k; ++j) {
        int x = (k*i+j) % blockDim.x;//TODO blockDim.x must be equal to grid size in one dim
        int y = ((k*i+j) / blockDim.x) % blockDim.x;
        int z = ((k*i+j) / blockDim.x / blockDim.x) % blockDim.x;

        grid.densities[k*i+j] = noise_func(offset + make_float3(x * a, y * a, z * a));
    }


}

__global__ void calc_cells(Grid grid, float3 offset, float a){
    int i = blockIdx.x * blockDim.x + threadIdx.x;

    int x = i % blockDim.x;
    int y = (i / blockDim.x) % blockDim.x;
    int z = (i / blockDim.x / blockDim.x) % blockDim.x;

    //if(x >= 128 || y >= 128 || z >= 128) return;

    float d00 = grid.densities[i];
    float d01 = grid.densities[i + 1];
    float d02 = grid.densities[(z+1) * blockDim.x * blockDim.x + y * blockDim.x + x + 1];
    float d03 = grid.densities[(z+1) * blockDim.x * blockDim.x + y * blockDim.x + x];

    float d10 = grid.densities[z * blockDim.x * blockDim.x + (y+1) * blockDim.x + x];
    float d11 = grid.densities[z * blockDim.x * blockDim.x + (y+1) * blockDim.x + x + 1];
    float d12 = grid.densities[(z+1) * blockDim.x * blockDim.x + (y+1) * blockDim.x + x + 1];
    float d13 = grid.densities[(z+1) * blockDim.x * blockDim.x + (y+1) * blockDim.x + x];

    bool e0001 = density_change(d00,d01);
    bool e0102 = density_change(d01,d02);
    bool e0203 = density_change(d02,d03);
    bool e0300 = density_change(d03,d00);

    bool e1011 = density_change(d10,d11);
    bool e1112 = density_change(d11,d12);
    bool e1213 = density_change(d12,d13);
    bool e1310 = density_change(d13,d10);


    bool e0010 = density_change(d00,d10);
    bool e0111 = density_change(d01,d11);
    bool e0212 = density_change(d02,d12);
    bool e0313 = density_change(d03,d13);


    int config = 0;

    config |= e0001;
    config |= e0102 << 1;
    config |= e0203 << 2;
    config |= e0300 << 3;

    config |= e1011 << 4;
    config |= e1112 << 5;
    config |= e1213 << 6;
    config |= e1310 << 7;

    config |= e0010 << 8;
    config |= e0111 << 9;
    config |= e0212 << 10;
    config |= e0313 << 11;

    grid.cells[i].config = config;

//
//    if(e0001){
//        float3 start = offset + make_float3(x * a, y * a, z * a);
//        float3 end = offset + make_float3((x+1) * a, y * a, z * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[0] = intersection;
//    }
//
//    if(e0102){
//
//        float3 start = offset + make_float3((x+1) * a, y * a, z * a);
//        float3 end = offset + make_float3((x+1) * a, y * a, (z+1) * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[1] = intersection;
//    }
//
//    if(e0203){
//
//        float3 start = offset + make_float3((x+1) * a, y * a, (z+1) * a);
//        float3 end = offset + make_float3(x * a, y * a, (z+1) * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[2] = intersection;
//    }
//
//    if(e0300){
//
//        float3 start = offset + make_float3(x * a, y * a, (z+1) * a);
//        float3 end = offset + make_float3(x * a, y * a, z * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[3] = intersection;
//    }
//
//
//
//
//
//
//
//
//    if(e1011){
//        float3 start = offset + make_float3(x * a, (y+1) * a, z * a);
//        float3 end = offset + make_float3((x+1) * a, (y+1) * a, z * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[4] = intersection;
//    }
//
//    if(e1112){
//
//        float3 start = offset + make_float3((x+1) * a, (y+1) * a, z * a);
//        float3 end = offset + make_float3((x+1) * a, (y+1) * a, (z+1) * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[5] = intersection;
//    }
//
//    if(e1213){
//
//        float3 start = offset + make_float3((x+1) * a, (y+1) * a, (z+1) * a);
//        float3 end = offset + make_float3(x * a, (y+1) * a, (z+1) * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[6] = intersection;
//    }
//
//    if(e1310){
//
//        float3 start = offset + make_float3(x * a, (y+1) * a, (z+1) * a);
//        float3 end = offset + make_float3(x * a, (y+1) * a, z * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[7] = intersection;
//    }
//
//
//    if(e0010){
//        float3 start = offset + make_float3(x * a, y * a, z * a);
//        float3 end = offset + make_float3(x * a, (y+1) * a, z * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[8] = intersection;
//    }
//
//    if(e0111){
//
//        float3 start = offset + make_float3((x+1) * a, y * a, z * a);
//        float3 end = offset + make_float3((x+1) * a, (y+1) * a, z * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[9] = intersection;
//    }
//
//    if(e0212){
//
//        float3 start = offset + make_float3((x+1) * a, y * a, (z+1) * a);
//        float3 end = offset + make_float3((x+1) * a, (y+1) * a, (z+1) * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[10] = intersection;
//    }
//
//    if(e0313){
//
//        float3 start = offset + make_float3(x * a, y * a, (z+1) * a);
//        float3 end = offset + make_float3(x * a, (y+1) * a, (z+1) * a);
//
//        Line line{start,end};
//        float3 intersection = sample_intersection(line);
//
//        grid.cells[i].intersections[11] = intersection;
//    }



}


#define gpuErrchk(ans) { gpuAssert((ans), __FILE__, __LINE__); }
inline void gpuAssert(cudaError_t code, const char *file, int line, bool abort=true)
{
    if (code != cudaSuccess)
    {
        fprintf(stderr,"GPUassert: %s %s %d\n", cudaGetErrorString(code), file, line);
        if (abort) exit(code);
    }
}


int getSPcores(cudaDeviceProp devProp)
{
    int cores = 0;
    int mp = devProp.multiProcessorCount;
    switch (devProp.major){
        case 2: // Fermi
            if (devProp.minor == 1) cores = mp * 48;
            else cores = mp * 32;
            break;
        case 3: // Kepler
            cores = mp * 192;
            break;
        case 5: // Maxwell
            cores = mp * 128;
            break;
        case 6: // Pascal
            if (devProp.minor == 1) cores = mp * 128;
            else if (devProp.minor == 0) cores = mp * 64;
            else printf("Unknown device type\n");
            break;
        case 7: // Volta
            if (devProp.minor == 0) cores = mp * 64;
            else printf("Unknown device type\n");
            break;
        default:
            printf("Unknown device type\n");
            break;
    }
    return cores;
}

