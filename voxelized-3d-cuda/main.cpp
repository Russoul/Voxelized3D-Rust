#include <string>
#include <iostream>
#include <vector>
#include "FastNoise.h"
#include <cmath>
#include "helper_math.h"


/*
struct float3{
    float x;
    float y;
    float z;
};

float3 operator+(float3 a, float3 b){
    return {a.x + b.x, a.y + b.y, a.z + b.z};
}


float3 operator*(float3 a, float k){
    return {a.x * k, a.y * k, a.z * k};
}

float3 make_float3(float x, float y, float z){
    return {x,y,z};
}
*/




std::string dump_float3(float3 a){
    return "( " + std::to_string(a.x) + ", " + std::to_string(a.y) + ", " + std::to_string(a.z) + " )";
}




struct Node{
    Node *parent = nullptr;
    Node *children[8] = {0,0,0,0,0,0,0,0};
    float density = 0.0F;
};






FastNoise noise;

inline float noise_func(float3 pos){
    auto val = noise.GetValue(pos.x, pos.y, pos.z);
    //std::cout << val << std::endl;
    return val;
}


void simplify_grid2(Node **dense_grid, Node **simplified, int cur_size){


    int last_size = cur_size * 2;

    for (int z = 0; z < cur_size; ++z) {
        for (int y = 0; y < cur_size; ++y) {
            for (int x = 0; x < cur_size; ++x) {
                int i = z * cur_size * cur_size + y * cur_size + x; //in new grid

                int a0 = 2*z * last_size * last_size + 2*y * last_size + 2*x;
                int a1 = 2*z * last_size * last_size + 2*y * last_size + 2*x+1;
                int a2 = (2*z+1) * last_size * last_size + 2*y * last_size + 2*x+1;
                int a3 = (2*z+1) * last_size * last_size + 2*y * last_size + 2*x;

                int a4 = 2*z * last_size * last_size + (2*y+1) * last_size + 2*x;
                int a5 = 2*z * last_size * last_size + (2*y+1) * last_size + 2*x+1;
                int a6 = (2*z+1) * last_size * last_size + (2*y+1) * last_size + 2*x+1;
                int a7 = (2*z+1) * last_size * last_size + (2*y+1) * last_size + 2*x;

                int as[] = {a0,a1,a2,a3,a4,a5,a6,a7};

                float i0 = dense_grid[a0]->density;
                float i1 = dense_grid[a1]->density;
                float i2 = dense_grid[a2]->density;
                float i3 = dense_grid[a3]->density;

                float i4 = dense_grid[a4]->density;
                float i5 = dense_grid[a5]->density;
                float i6 = dense_grid[a6]->density;
                float i7 = dense_grid[a7]->density;


                //TODO incorrect for float density
                if(i0 == i1 && i1 == i2 && i2 == i3 && i3 == i4 && i4 == i5 && i5 == i6 && i6 == i7
                        && dense_grid[a0]->children[0] == nullptr &&
                        dense_grid[a1]->children[0] == nullptr &&
                        dense_grid[a2]->children[0] == nullptr &&
                        dense_grid[a3]->children[0] == nullptr &&
                        dense_grid[a4]->children[0] == nullptr &&
                        dense_grid[a5]->children[0] == nullptr &&
                        dense_grid[a6]->children[0] == nullptr &&
                        dense_grid[a7]->children[0] == nullptr){
                    simplified[i]->density = i0;
                    for (int j = 0; j < 8; ++j) {
                        simplified[i]->children[j] = 0;
                        //printf("deleting %d out of %d\n", as[j], last_size * last_size * last_size);
                        delete dense_grid[as[j]];
                    }
                }else{
                    simplified[i]->density = -1;
                    for (int j = 0; j < 8; ++j) {
                        simplified[i]->children[j] = dense_grid[as[j]];
                        dense_grid[as[j]]->parent = simplified[i];
                    }
                }
            }
        }
    }


}







float3 centers[] = {make_float3(-0.5F, -0.5F, -0.5F), make_float3(0.5F, -0.5F, -0.5F), make_float3(0.5F, -0.5F, 0.5F), make_float3(-0.5F, -0.5F, 0.5F),
                    make_float3(-0.5F, 0.5F, -0.5F), make_float3(0.5F, 0.5F, -0.5F), make_float3(0.5F, 0.5F, 0.5F), make_float3(-0.5F, 0.5F, 0.5F)};



template<class Func>
void for_each_leaf(Node *root, Func f, float3 center, float extent, int level = 0){

    if(root == nullptr) return;

    bool leaf = true;
    for (int i = 0; i < 8; ++i) { //TODO switch this to one single check: type != -1
        if(root->children[i] != nullptr){
            leaf = false;
            break;
        }
    }

    if(leaf){
        f(root, center, extent, level);
    }else{
        for (int i = 0; i < 8; ++i) {
            auto new_center = centers[i] * extent + center;
            for_each_leaf(root->children[i], f, new_center,extent/2, level + 1);
        }
    }

}



void gen_triangles_from_box(float3 center, float extent, std::vector<float> &vertices, std::vector<uint> &indices){

    float3 corners[8];

    for (int i = 0; i < 8; ++i) {
        corners[i] = centers[i] * 2 * extent + center;

    }

    vertices.push_back(corners[0].x);
    vertices.push_back(corners[0].y);
    vertices.push_back(corners[0].z);

    vertices.push_back(0);
    vertices.push_back(-1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);

    vertices.push_back(corners[1].x);
    vertices.push_back(corners[1].y);
    vertices.push_back(corners[1].z);

    vertices.push_back(0);
    vertices.push_back(-1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);

    vertices.push_back(corners[2].x);
    vertices.push_back(corners[2].y);
    vertices.push_back(corners[2].z);

    vertices.push_back(0);
    vertices.push_back(-1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[3].x);
    vertices.push_back(corners[3].y);
    vertices.push_back(corners[3].z);

    vertices.push_back(0);
    vertices.push_back(-1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    //


    vertices.push_back(corners[4].x);
    vertices.push_back(corners[4].y);
    vertices.push_back(corners[4].z);

    vertices.push_back(0);
    vertices.push_back(1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[5].x);
    vertices.push_back(corners[5].y);
    vertices.push_back(corners[5].z);

    vertices.push_back(0);
    vertices.push_back(1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[6].x);
    vertices.push_back(corners[6].y);
    vertices.push_back(corners[6].z);

    vertices.push_back(0);
    vertices.push_back(1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[7].x);
    vertices.push_back(corners[7].y);
    vertices.push_back(corners[7].z);

    vertices.push_back(0);
    vertices.push_back(1);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);



    //


    vertices.push_back(corners[7].x); //8
    vertices.push_back(corners[7].y);
    vertices.push_back(corners[7].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);

    vertices.push_back(corners[6].x); //9
    vertices.push_back(corners[6].y);
    vertices.push_back(corners[6].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);



    vertices.push_back(corners[2].x); //10
    vertices.push_back(corners[2].y);
    vertices.push_back(corners[2].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[3].x); //11
    vertices.push_back(corners[3].y);
    vertices.push_back(corners[3].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);

    //

    vertices.push_back(corners[4].x); //12
    vertices.push_back(corners[4].y);
    vertices.push_back(corners[4].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(-1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[5].x); //13
    vertices.push_back(corners[5].y);
    vertices.push_back(corners[5].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(-1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);



    vertices.push_back(corners[1].x); //14
    vertices.push_back(corners[1].y);
    vertices.push_back(corners[1].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(-1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[0].x); //15
    vertices.push_back(corners[0].y);
    vertices.push_back(corners[0].z);

    vertices.push_back(0);
    vertices.push_back(0);
    vertices.push_back(-1);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);

    //

    vertices.push_back(corners[7].x); //16
    vertices.push_back(corners[7].y);
    vertices.push_back(corners[7].z);

    vertices.push_back(-1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    vertices.push_back(corners[4].x); //17
    vertices.push_back(corners[4].y);
    vertices.push_back(corners[4].z);

    vertices.push_back(-1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);



    vertices.push_back(corners[0].x); //18
    vertices.push_back(corners[0].y);
    vertices.push_back(corners[0].z);

    vertices.push_back(-1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);



    vertices.push_back(corners[3].x); //19
    vertices.push_back(corners[3].y);
    vertices.push_back(corners[3].z);

    vertices.push_back(-1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    //


    vertices.push_back(corners[6].x); //20
    vertices.push_back(corners[6].y);
    vertices.push_back(corners[6].z);

    vertices.push_back(1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);

    vertices.push_back(corners[5].x); //21
    vertices.push_back(corners[5].y);
    vertices.push_back(corners[5].z);

    vertices.push_back(1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);



    vertices.push_back(corners[1].x); //22
    vertices.push_back(corners[1].y);
    vertices.push_back(corners[1].z);

    vertices.push_back(1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);



    vertices.push_back(corners[2].x); //23
    vertices.push_back(corners[2].y);
    vertices.push_back(corners[2].z);

    vertices.push_back(1);
    vertices.push_back(0);
    vertices.push_back(0);

    vertices.push_back(1);
    vertices.push_back(1);
    vertices.push_back(1);


    uint ind[] = {2,3,0, 2,0,1, 4,7,6, 4,6,5, 10,9,8, 10,8,11, 12,14,15, 12,13,14, 16,18,19, 16,17,18, 20,23,22, 20,22,21 };

    uint count = vertices.size()/9;

    for (int j = 0; j < 36; ++j) {
        indices.push_back(ind[j] + count);
    }
}


std::string spaces(int count){
    std::string s = "";
    for (int i = 0; i < count; ++i) {
        s += " ";
    }

    return s;
}




/*void print_device_info(){
    //========================== DEVICE INFO =============================
    int            deviceCount;
    cudaDeviceProp devProp;

    auto err = cudaGetDeviceCount ( &deviceCount );

    std::cout << cudaGetErrorString(err) << std::endl;

    printf ( "Found %d devices\n", deviceCount );

    std::cout << "using the first one" << std::endl;


    cudaGetDeviceProperties ( &devProp, 0 );

    printf("Device name:                %s\n", devProp.name);
    printf("Major revision number:      %d\n", devProp.major);
    printf("Minor revision Number:      %d\n", devProp.minor);
    printf("Total Global Memory:        %d\n", devProp.totalGlobalMem);
    printf("Total shared mem per block: %d\n", devProp.sharedMemPerBlock);
    printf("Total const mem size:       %d\n", devProp.totalConstMem);
    printf("Warp size:                  %d\n", devProp.warpSize);
    printf("Maximum block dimensions:   %d x %d x %d\n", devProp.maxThreadsDim[0], \
                                                                                                          devProp.maxThreadsDim[1], \
                                                                                                          devProp.maxThreadsDim[2]);

    printf("Maximum grid dimensions:    %d x %d x %d\n", devProp.maxGridSize[0], \
                                                                                                          devProp.maxGridSize[1], \
                                                                                                          devProp.maxGridSize[2]);
    printf("Clock Rate:                 %d\n", devProp.clockRate);
    printf("Number of muliprocessors:   %d\n", devProp.multiProcessorCount);

    printf("number of cores %d\n", getSPcores(devProp));
    //====================================================================
}*/


extern "C" void print_octree(Node *node, int lev = 0){
    auto sp = spaces(2 * lev);
    if(node->children[0] == nullptr){
        printf("%s%f\n", sp.c_str(), node->density);
    }else{
        printf("%s*\n", sp.c_str());
        for (auto &i : node->children) {
            print_octree(i, lev + 1);
        }
    }
}

extern "C" void print_node(Node* node){
    if(node){
        printf("(type=%f, children = {%u %u %u %u %u %u %u %u})\n", node->density, node->children[0],node->children[1],node->children[2],node->children[3],node->children[4],node->children[5],node->children[6],node->children[7]);
    }else{
        printf("null\n");
    }
}

extern "C" void alloc_grid(uint size, Node ***grid){
    *grid = new Node*[size * size * size];
    for (int i = 0; i < size * size * size; ++i) {
        (*grid)[i] = new Node;
    }
}

extern "C" void gen_dense_grid(uint size, float3 center, float chunk_extent, Node **dense_grid){


    float block_extent = chunk_extent / size;

    for (int z = 0; z < size; ++z) {
        for (int y = 0; y < size; ++y) {
            for (int x = 0; x < size; ++x) {
                int i = z * size * size + y * size + x;

                dense_grid[i]->density = noise_func(center + make_float3(-chunk_extent + block_extent + block_extent * 2 * x, -chunk_extent + block_extent + block_extent * 2 * y,-chunk_extent + block_extent + block_extent * 2 * z));
                dense_grid[i]->children[0] = 0;
                dense_grid[i]->children[1] = 0;
                dense_grid[i]->children[2] = 0;
                dense_grid[i]->children[3] = 0;

                dense_grid[i]->children[4] = 0;
                dense_grid[i]->children[5] = 0;
                dense_grid[i]->children[6] = 0;
                dense_grid[i]->children[7] = 0;
            }
        }
    }
}


extern "C" void gen_dense_grid_custom(uint size, float3 center, float chunk_extent, Node **dense_grid, float(*f)(void*, float3), void* arg){


    float block_extent = chunk_extent / size;

    for (int z = 0; z < size; ++z) {
        for (int y = 0; y < size; ++y) {
            for (int x = 0; x < size; ++x) {
                int i = z * size * size + y * size + x;

                dense_grid[i]->density = f(arg, center + make_float3(-chunk_extent + block_extent + block_extent * 2 * x, -chunk_extent + block_extent + block_extent * 2 * y,-chunk_extent + block_extent + block_extent * 2 * z));
                dense_grid[i]->children[0] = 0;
                dense_grid[i]->children[1] = 0;
                dense_grid[i]->children[2] = 0;
                dense_grid[i]->children[3] = 0;

                dense_grid[i]->children[4] = 0;
                dense_grid[i]->children[5] = 0;
                dense_grid[i]->children[6] = 0;
                dense_grid[i]->children[7] = 0;
            }
        }
    }
}

extern "C" Node **simplify_grid_recursively(uint size, float3 center, float extent, Node **dense_grid){
    Node **simpl = dense_grid;

    uint cur_size = size / 2;

    while(cur_size != 0){
        Node **grid;
        alloc_grid(cur_size, &grid);


        simplify_grid2(simpl, grid, cur_size);


        /*for (int i = 0; i < cur_size * cur_size * cur_size; ++i) {
            print_node(grid[i]);
        }*/

        simpl = grid;


        cur_size /= 2;
    }

    return simpl;


}

extern "C" void init_noise(){
    noise.SetNoiseType(FastNoise::Perlin);
    noise.SetFrequency(4);
}


/*int main(){



    uint size = 64;
    float3 center = make_float3(0,0,0);
    float extent = 0.5F;
    Node **dense_grid;
    alloc_grid(size, &dense_grid);


    gen_dense_grid(size, center, extent, dense_grid);



    auto **res = simplify_grid_recursively(size, center, extent, dense_grid);


    printf("res\n");
    print_octree(res[0]);

    //TODO deletion


    return 0;
}*/

int main1(){





    /*uint size = 64; //num of blocks in one dimension

    Node *nodes_max_res_d;
    cudaMalloc(&nodes_max_res_d, sizeof(Node)*size*size*size);




    cudaEvent_t start, stop;
    float elapsedTime;

    cudaEventCreate(&start);
    cudaEventRecord(start,0);


    float3 center = make_float3(0,0,0);
    float extent = 0.5F;

    construct_grid<<<size*size,size>>>(nodes_max_res_d, center, extent, extent/size);

    Node *nodes_max_res = static_cast<Node *>(malloc(sizeof(Node) * size * size * size));
    cudaMemcpy(nodes_max_res, nodes_max_res_d, sizeof(Node)*size*size*size, cudaMemcpyDeviceToHost);

    int cur_size = size/2;


    Node *last_simpl = nodes_max_res;

    //TODO learn about GPU scans

    while(cur_size != 0){
        Node *nodes = static_cast<Node *>(malloc(sizeof(Node) * cur_size * cur_size * cur_size));
        //Node *nodes = new Node[cur_size * cur_size * cur_size];

        simplify_grid(last_simpl, nodes, cur_size);

        last_simpl = nodes;

        if(cur_size == 64){
            for (int i = 0; i < cur_size * cur_size * cur_size; ++i) {
                print_node(nodes + i);
            }
        }

        cur_size /= 2;
    }


    std::vector<float> vertices;
    std::vector<uint> indices;

    for_each_leaf(last_simpl, [&](Node *node, float3 center, float extent, int level){
        if(node->type == 2) gen_triangles_from_box(center, extent, vertices, indices);
    }, center, extent);


    printf("type %d\n", last_simpl->type);
    printf("children4 %d %d %d %d\n", last_simpl->children[0], last_simpl->children[1], last_simpl->children[2], last_simpl->children[3]);

    cms::Mesh mesh(vertices, indices);

    mesh.exportOBJ("test.obj");

    cudaEventCreate(&stop);
    cudaEventRecord(stop,0);
    cudaEventSynchronize(stop);


    cudaEventElapsedTime(&elapsedTime, start,stop);
    printf("Elapsed time : %f ms\n" ,elapsedTime);

    gpuErrchk( cudaPeekAtLastError() );*/






    return 0;
}
