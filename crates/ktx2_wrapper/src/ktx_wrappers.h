#pragma once

#include "ktx.h"
#include <memory>

struct KtxBasisParams;
struct KtxBasisUASTCParams;
struct KtxBasisETC1SParams;

ktxTexture2*
ktxTexture2_CreateWrapped(
    ktxTextureCreateInfo* const createInfo,
    ktxTextureCreateStorageEnum storageAllocation,
    ktx_error_code_e* return_error_code
);

KTX_error_code ktxTexture_SetImageFromMemoryWrapped(
    ktxTexture2* texture,
    ktx_uint32_t level,
    ktx_uint32_t layer,
    ktx_uint32_t faceSlice,
    unsigned char* file_data,
    ktx_size_t image_size
);

KTX_error_code ktxTexture_WriteToNamedFileWrapped(ktxTexture2* texture, const char* filename);

void ktxTexture_DestroyWrapped(ktxTexture2* texture);

KTX_error_code ktxTexture2_CompressAstcExWrapped(ktxTexture2* texture, std::unique_ptr<ktxAstcParams> params);

ktxBasisParams intoCPPBasisUastcParams(const KtxBasisParams* basisParams, const KtxBasisUASTCParams* uastcParams);

KTX_error_code ktxTexture2_CompressBasisUastc(ktxTexture2* texture, const KtxBasisParams* basisParams, const KtxBasisUASTCParams* uastcParams);

ktxBasisParams intoCPPBasisEtc1sParams(const KtxBasisParams* basisParams, const KtxBasisETC1SParams* etc1sParams);

KTX_error_code ktxTexture2_CompressBasisEtc1s(ktxTexture2* texture, const KtxBasisParams* basisParams, const KtxBasisETC1SParams* etc1sParams);
