//////////////////////////////////////////////
/// @file types.h
/// @brief a typdefining header
///
/// The Cubical Marching Squares (CMS) Project
/// licensed under the 3 clause BSD licence
/// found in LICENCE.md 2015
///
/// @author George Rassovsky
/// (goro.rassovsky@gmail.com)
//////////////////////////////////////////////

#ifndef CMS_TYPES_H
#define CMS_TYPES_H

#include <vector>
#include <stdint.h>

namespace cms
{

typedef unsigned int uint;
typedef unsigned long long int ullint;
typedef std::vector<uint> uintVec;
typedef std::vector<uintVec> uintVecVec;
typedef std::vector<int> intVec;
typedef std::vector<int8_t> int8Vec;
typedef std::vector<float> floatVec;

}

#endif //CMS_TYPES_H
