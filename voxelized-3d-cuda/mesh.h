//////////////////////////////////////////////
/// @file mesh.h
/// @brief Mesh class header
///
/// The Cubical Marching Squares (CMS) Project
/// licensed under the 3 clause BSD licence
/// found in LICENCE.md 2015
///
/// @author George Rassovsky
/// (goro.rassovsky@gmail.com)
//////////////////////////////////////////////

#ifndef CMS_MESH_H
#define CMS_MESH_H

#include "types.h"
#include <vector>
#include <string>

namespace cms
{


class Mesh
{

public:
  /// @brief Default constructor
  Mesh();
  /// @brief Constructor with supplied vertices and indices
  Mesh(floatVec i_vertices, uintVec i_indices);
  /// @brief Constructor with supplied vertices and indices and normals
  Mesh(floatVec i_vertices, floatVec i_normals, uintVec i_indices);

  /// @brief Accessors and Mutators
  inline void setVertices(floatVec i_vertices) { m_vertices = i_vertices; }
  inline floatVec getVertices() const { return m_vertices; }

  inline void setIndices(uintVec i_indices) { m_indices = i_indices; }
  inline uintVec getIndices() const { return m_indices; }

  inline void setNormals(floatVec i_normals) { m_normals = i_normals; }
  inline floatVec getNormals() const { return m_normals; }

  /// @brief Returns the number of vertices in the mesh
  uint vertexCount() const;

  /// @brief Returns the number of indices in the mesh
  uint indexCount() const;

  /// @brief Returns the number of faces in the mesh /triangles/
  uint faceCount() const;

  /// @brief push vertex on the end of the mesh vertex vector
  void pushVertex(float i_x, float i_y, float i_z);

  /// @brief push normal on the end of the mesh normal vector
  void pushNormal(float i_x, float i_y, float i_z);

  /// @brief push index on the end of the mesh index vector
  void pushIndex(int i_ind);

  /// @todo float* getVertexAt(int i_index); /// same for normal and index

  /// @brief Populates a float array with the extreme coordinates of the mesh
  /// in the following format: {+x +y +z -x -y -z}
  /// @param takes in a pointer to an array of 6 floats
  void getBoundingBox(float* i_bbox) const;

  /// @brief Exports the mesh as a Wavefront OBJ file
  /// @param Takes in the name or path/and/name of the file
  bool exportOBJ(const std::string &i_fName) const;


private:
  floatVec m_vertices;
  floatVec m_normals;
  uintVec m_indices;
};


} //namespace cms

#endif //CMS_MESH_H
