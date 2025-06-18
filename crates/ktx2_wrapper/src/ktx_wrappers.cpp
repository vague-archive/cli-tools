#include "ktx_wrappers.h"
#include "cxxgen1.h"

ktxTexture2*
ktxTexture2_CreateWrapped(
    ktxTextureCreateInfo* const createInfo,
    ktxTextureCreateStorageEnum storageAllocation,
    ktx_error_code_e* return_error_code
) {
    ktxTexture2* ktx_texture;
    *return_error_code = ktxTexture2_Create(createInfo, storageAllocation, &ktx_texture);
    return ktx_texture;
}

KTX_error_code ktxTexture_SetImageFromMemoryWrapped(
    ktxTexture2* texture,
    ktx_uint32_t level,
    ktx_uint32_t layer,
    ktx_uint32_t faceSlice,
    unsigned char* file_data,
    ktx_size_t image_size
) {
    return ktxTexture_SetImageFromMemory((ktxTexture*)texture, level, layer, faceSlice, file_data, image_size);
}

KTX_error_code ktxTexture_WriteToNamedFileWrapped(ktxTexture2* texture, const char* filename) {
    return ktxTexture_WriteToNamedFile((ktxTexture*)texture, filename);
}

void ktxTexture_DestroyWrapped(ktxTexture2* texture) {
    return ktxTexture_Destroy((ktxTexture*)texture);
}

KTX_error_code ktxTexture2_CompressAstcExWrapped(ktxTexture2* texture, std::unique_ptr<ktxAstcParams> params) {
    params->structSize = sizeof(*params);
    return ktxTexture2_CompressAstcEx(texture, params.get());
}

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"

ktxBasisParams intoCPPBasisUastcParams(const KtxBasisParams* basisParams, const KtxBasisUASTCParams* uastcParams) {
    ktxBasisParams output = { .uastc = basisParams->uastc };
    output.structSize = sizeof(output);
    if (basisParams->verbose) {
        output.verbose = *basisParams->verbose;
    }
    if (basisParams->no_sse) {
        output.noSSE = *basisParams->no_sse;
    }
    if (basisParams->thread_count) {
        output.threadCount = *basisParams->thread_count;
    }
    if (uastcParams->uastc_flags) {
        output.uastcFlags = uastcParams->uastc_flags->value;
    }
    if (uastcParams->uastc_rdo) {
        output.uastcRDO = *uastcParams->uastc_rdo;
    }
    if (uastcParams->uastc_rdo_quality_scalar) {
        output.uastcRDOQualityScalar = *uastcParams->uastc_rdo_quality_scalar;
    }
    if (uastcParams->uastc_rdo_dict_size) {
        output.uastcRDODictSize = *uastcParams->uastc_rdo_dict_size;
    }
    if (uastcParams->uastc_rdo_max_smooth_block_error_scale) {
        output.uastcRDOMaxSmoothBlockErrorScale = *uastcParams->uastc_rdo_max_smooth_block_error_scale;
    }
    if (uastcParams->uastc_rdo_max_smooth_block_std_dev) {
        output.uastcRDOMaxSmoothBlockStdDev = *uastcParams->uastc_rdo_max_smooth_block_std_dev;
    }
    if (uastcParams->uastc_rdo_dont_favor_simpler_modes) {
        output.uastcRDODontFavorSimplerModes = *uastcParams->uastc_rdo_dont_favor_simpler_modes;
    }
    if (uastcParams->uastc_rdo_no_multithreading) {
        output.uastcRDONoMultithreading = *uastcParams->uastc_rdo_no_multithreading;
    }
    if (uastcParams->input_swizzle && uastcParams->input_swizzle->length() > 4) {
        output.inputSwizzle[0] = uastcParams->input_swizzle->data()[0];
        output.inputSwizzle[1] = uastcParams->input_swizzle->data()[1];
        output.inputSwizzle[2] = uastcParams->input_swizzle->data()[2];
        output.inputSwizzle[3] = uastcParams->input_swizzle->data()[3];
    }
    if (uastcParams->pre_swizzle) {
        output.preSwizzle = uastcParams->pre_swizzle;
    }
    return output;
}

KTX_error_code ktxTexture2_CompressBasisUastc(ktxTexture2* texture, const KtxBasisParams* basisParams, const KtxBasisUASTCParams* uastcParams) {
    ktxBasisParams params = intoCPPBasisUastcParams(basisParams, uastcParams);
    return ktxTexture2_CompressBasisEx(texture, &params);
}

ktxBasisParams intoCPPBasisEtc1sParams(const KtxBasisParams* basisParams, const KtxBasisETC1SParams* etc1sParams) {
    ktxBasisParams output = { .uastc = basisParams->uastc };
    output.structSize = sizeof(output);
    if (basisParams->verbose) {
        output.verbose = *basisParams->verbose;
    }
    if (basisParams->no_sse) {
        output.noSSE = *basisParams->no_sse;
    }
    if (basisParams->thread_count) {
        output.threadCount = *basisParams->thread_count;
    }
    if (etc1sParams->compression_level) {
        output.compressionLevel = *etc1sParams->compression_level;
    }
    if (etc1sParams->quality_level) {
        output.qualityLevel = *etc1sParams->quality_level;
    }
    if (etc1sParams->max_endpoints) {
        output.maxEndpoints = *etc1sParams->max_endpoints;
    }
    if (etc1sParams->endpoint_rdo_threshold) {
        output.endpointRDOThreshold = *etc1sParams->endpoint_rdo_threshold;
    }
    if (etc1sParams->max_selectors) {
        output.maxSelectors = *etc1sParams->max_selectors;
    }
    if (etc1sParams->selector_rdo_threshold) {
        output.selectorRDOThreshold = *etc1sParams->selector_rdo_threshold;
    }
    if (etc1sParams->input_swizzle && etc1sParams->input_swizzle->length() > 4) {
        output.inputSwizzle[0] = etc1sParams->input_swizzle->data()[0];
        output.inputSwizzle[1] = etc1sParams->input_swizzle->data()[1];
        output.inputSwizzle[2] = etc1sParams->input_swizzle->data()[2];
        output.inputSwizzle[3] = etc1sParams->input_swizzle->data()[3];
    }
    if (etc1sParams->normal_map) {
        output.normalMap = *etc1sParams->normal_map;
    }
    if (etc1sParams->separate_rgt_to_rgba) {
        output.separateRGToRGB_A = *etc1sParams->separate_rgt_to_rgba;
    }
    if (etc1sParams->pre_swizzle) {
        output.preSwizzle = *etc1sParams->pre_swizzle;
    }
    if (etc1sParams->no_endpoint_rdo) {
        output.noEndpointRDO = *etc1sParams->no_selector_rdo;
    }
    if (etc1sParams->no_selector_rdo) {
        output.noSelectorRDO = *etc1sParams->no_selector_rdo;
    }

    return output;
}

#pragma GCC diagnostic pop

KTX_error_code ktxTexture2_CompressBasisEtc1s(ktxTexture2* texture, const KtxBasisParams* basisParams, const KtxBasisETC1SParams* etc1sParams) {
    ktxBasisParams params = intoCPPBasisEtc1sParams(basisParams, etc1sParams);
    return ktxTexture2_CompressBasisEx(texture, &params);
}
