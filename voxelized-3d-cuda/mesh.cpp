//////////////////////////////////////////////
/// @file mesh.cpp
/// @brief Mesh class implementation
///
/// The Cubical Marching Squares (CMS) Project
/// licensed under the 3 clause BSD licence
/// found in LICENCE.md 2015
///
/// @author George Rassovsky
/// (goro.rassovsky@gmail.com)
//////////////////////////////////////////////

#include "mesh.h"
#include <iostream>
#include <fstream>
#include <limits>
#include <stdlib.h> //realpath
#include <sys/stat.h> //mkdir

namespace cms
{

Mesh::Mesh()
{
}

//-----------------------------------------------------------------------------------------

Mesh::Mesh(std::vector<float> i_vertices, std::vector<unsigned int> i_indices) :
   m_vertices(i_vertices), m_indices(i_indices)
{
}

//-----------------------------------------------------------------------------------------

Mesh::Mesh(std::vector<float> i_vertices, std::vector<float> i_normals, std::vector<unsigned int> i_indices) :
  m_vertices(i_vertices), m_normals(i_normals), m_indices(i_indices)
{
}

//-----------------------------------------------------------------------------------------

unsigned int Mesh::vertexCount() const
{
  return m_vertices.size()/3;
}

//-----------------------------------------------------------------------------------------

unsigned int Mesh::indexCount() const
{
  return m_indices.size();
}

//-----------------------------------------------------------------------------------------

unsigned int Mesh::faceCount() const
{
  return m_indices.size()/3;
}

//-----------------------------------------------------------------------------------------

void Mesh::getBoundingBox(float* i_bbox) const
{
  // Assigning the bbox to some improbable limits
  i_bbox[0] = i_bbox[1] = i_bbox[2] = std::numeric_limits<float>::max();
  i_bbox[3] = i_bbox[4] = i_bbox[5] = -(std::numeric_limits<float>::max());

  // Finding the X limits
  for(unsigned i=0; i<m_vertices.size(); i+=3)
  {
    i_bbox[0] = std::min(i_bbox[0], m_vertices[i]);
    i_bbox[3] = std::max(i_bbox[3], m_vertices[i]);
  }

  // Finding the Y limits
  for(unsigned i=1; i<m_vertices.size(); i+=3)
  {
    i_bbox[1] = std::min(i_bbox[1], m_vertices[i]);
    i_bbox[4] = std::max(i_bbox[4], m_vertices[i]);
  }

  // Finding the Z limits
  for(unsigned i=0; i<m_vertices.size(); i+=3)
  {
    i_bbox[2] = std::min(i_bbox[2], m_vertices[i]);
    i_bbox[5] = std::max(i_bbox[5], m_vertices[i]);
  }
}

//-----------------------------------------------------------------------------------------

bool Mesh::exportOBJ( const std::string& i_fName ) const
{
  // Add output directory posix path to file name
  std::string outputDir = "output/";
  std::string fullPath = "";

  // Set the fullPath
  fullPath = outputDir + i_fName;

  // If the output folder doesn't exist try to create one
  // if creating it was unsuccessful then modify fullPath so it just outputs to the build root
  struct stat sb;
  if (stat(outputDir.c_str(), &sb) != 0 || !S_ISDIR(sb.st_mode)) //dir doesn't exist
  {
    if( mkdir(outputDir.c_str(), S_IRWXU | S_IRWXG | S_IROTH | S_IXOTH) == -1 )
    {
      std::cout<<"Directory '"<<outputDir<<"' could not be created! \n" <<
                  "The file will not be outputed in the root of the project build dir." <<
                  std::endl;

      fullPath = i_fName;
    }
  }

  // Open the stream and parse
  std::fstream fileOut;
  fileOut.open(fullPath.c_str(), std::ios::out);

  if(fileOut.is_open())
  {
    fileOut<<"# CMS Isosurface extraction."<<std::endl;
    fileOut<<"# George Rassovsky ~ goro.rassovsky@gmail.com \n"<<std::endl;

    for(unsigned int i=0; i<m_vertices.size(); i+=3)
    {
      fileOut<<"v "<<m_vertices[i]<<" "<<m_vertices[i+1]<<" "<<m_vertices[i+2]<<std::endl;
    }

    // Write the face info
    for(unsigned f=0; f<m_indices.size(); f+=3)
    {
      fileOut<<"f "<<m_indices[f]+1<<" "<<m_indices[f+1]+1<<" "<<m_indices[f+2]+1<<std::endl;
    }

    char * resolvedPath = realpath(fullPath.c_str(), nullptr);
    if(resolvedPath)
    {
      std::cout << "\nExported mesh path: " << resolvedPath << std::endl;
      free(resolvedPath);
    }
    else
    {
      std::cout << "\nFile error.\n";
    }

    return true;
  }
  else
  {
    std::cout <<"File : "<<i_fName<<" Not founds "<<std::endl;
    return false;
  }
}

//-----------------------------------------------------------------------------------------

void Mesh::pushVertex(float i_x, float i_y, float i_z)
{
  /// @todo maybe make it more efficient withought pushbacks
  m_vertices.push_back(i_x);
  m_vertices.push_back(i_y);
  m_vertices.push_back(i_z);
}

//-----------------------------------------------------------------------------------------

void Mesh::pushNormal(float i_x, float i_y, float i_z)
{
  m_normals.push_back(i_x);
  m_normals.push_back(i_y);
  m_normals.push_back(i_z);
}

//-----------------------------------------------------------------------------------------

void Mesh::pushIndex(int i_ind)
{
  m_indices.push_back(i_ind);
}

} //namespace cms
