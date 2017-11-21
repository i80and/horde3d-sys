pub type H3DRes = ::std::os::raw::c_int;
pub type H3DNode = ::std::os::raw::c_int;
pub const H3D_ROOT_NODE: H3DNode = 1;

#[repr(C)]
pub enum H3DRenderDevice {
    OpenGL2 = 2,
    OpenGL4 = 4,
}


#[repr(C)]
pub enum H3DOptions {
    MaxLogLevel = 1,
    MaxNumMessages,
    TrilinearFiltering,
    MaxAnisotropy,
    TexCompression,
    SRGBLinearization,
    LoadTextures,
    FastAnimation,
    ShadowMapSize,
    SampleCount,
    WireframeMode,
    DebugViewMode,
    DumpFailedShaders,
    GatherTimeStats,
}

#[repr(C)]
pub enum H3DStats {
    TriCount = 100,
    BatchCount,
    LightPassCount,
    FrameTime,
    AnimationTime,
    GeoUpdateTime,
    ParticleSimTime,
    FwdLightsGPUTime,
    DefLightsGPUTime,
    ShadowsGPUTime,
    ParticleGPUTime,
    TextureVMem,
    GeometryVMem,
    ComputeGPUTime,
}

#[repr(C)]
pub enum H3DDeviceCapabilities {
    GeometryShaders = 200,
    TessellationShaders,
    ComputeShaders,
}

#[repr(C)]
pub enum H3DResTypes {
    Undefined = 0,
    SceneGraph,
    Geometry,
    Animation,
    Material,
    Code,
    Shader,
    Texture,
    ParticleEffect,
    Pipeline,
    ComputeBuffer,
}

#[repr(C)]
pub enum H3DResFlags {
    NoQuery = 1,
    NoTexCompression = 2,
    NoTexMipmaps = 4,
    TexCubemap = 8,
    TexDynamic = 16,
    TexRenderable = 32,
    TexSRGB = 64,
}


#[repr(C)]
pub enum H3DFormats {
    Unknown = 0,
    TexBGRA8,
    TexDXT1,
    TexDXT3,
    TexDXT5,
    TexRGBA16F,
    TexRGBA32F,
}


#[repr(C)]
pub enum H3DGeoRes {
    GeometryElem = 200,
    GeoIndexCountI,
    GeoVertexCountI,
    GeoIndices16I,
    GeoIndexStream,
    GeoVertPosStream,
    GeoVertTanStream,
    GeoVertStaticStream,
}

#[repr(C)]
pub enum H3DAnimRes {
    EntityElem = 300,
    EntFrameCountI,
}

#[repr(C)]
pub enum H3DMatRes {
    MaterialElem = 400,
    SamplerElem,
    UniformElem,
    MatClassStr,
    MatLinkI,
    MatShaderI,
    SampNameStr,
    SampTexResI,
    UnifNameStr,
    UnifValueF4,
}

#[repr(C)]
pub enum H3DShaderRes {
    ContextElem = 600,
    SamplerElem,
    UniformElem,
    ContNameStr,
    SampNameStr,
    SampDefTexResI,
    UnifNameStr,
    UnifSizeI,
    UnifDefValueF4,
}

#[repr(C)]
pub enum H3DTexRes {
    TextureElem = 700,
    ImageElem,
    TexFormatI,
    TexSliceCountI,
    ImgWidthI,
    ImgHeightI,
    ImgPixelStream,
}

#[repr(C)]
pub enum H3DPartEffRes {
    ParticleElem = 800,
    ChanMoveVelElem,
    ChanRotVelElem,
    ChanSizeElem,
    ChanColRElem,
    ChanColGElem,
    ChanColBElem,
    ChanColAElem,
    PartLifeMinF,
    PartLifeMaxF,
    ChanStartMinF,
    ChanStartMaxF,
    ChanEndRateF,
    ChanDragElem,
}

#[repr(C)]
pub enum H3DPipeRes {
    StageElem = 900,
    StageNameStr,
    StageActivationI,
}

#[repr(C)]
pub enum H3DComputeBufRes {
    ComputeBufElem = 1000,
    DrawParamsElem,
    CompBufDataSizeI,
    CompBufDrawableI,
    DrawParamsNameStr,
    DrawParamsSizeI,
    DrawParamsOffsetI,
    DrawParamsCountI,
}

#[repr(C)]
pub enum H3DNodeTypes {
    Undefined = 0,
    Group,
    Model,
    Mesh,
    Joint,
    Light,
    Camera,
    Emitter,
    Compute,
}

#[repr(C)]
pub enum H3DNodeFlags {
    NoDraw = 1,
    NoCastShadow = 2,
    NoRayQuery = 4,
    Inactive = 7, // NoDraw | NoCastShadow | NoRayQuery
}

#[repr(C)]
pub enum H3DNodeParams {
    NameStr = 1,
    AttachmentStr,
}


#[repr(C)]
pub enum H3DModel {
    GeoResI = 200,
    SWSkinningI,
    LodDist1F,
    LodDist2F,
    LodDist3F,
    LodDist4F,
    AnimCountI,
}

#[repr(C)]
pub enum H3DMesh {
    MatResI = 300,
    BatchStartI,
    BatchCountI,
    VertRStartI,
    VertREndI,
    LodLevelI,
    TessellatableI,
}

#[repr(C)]
pub enum H3DJoint {
    JointIndexI = 400,
}

#[repr(C)]
pub enum H3DLight {
    MatResI = 500,
    RadiusF,
    FovF,
    ColorF3,
    ColorMultiplierF,
    ShadowMapCountI,
    ShadowSplitLambdaF,
    ShadowMapBiasF,
    LightingContextStr,
    ShadowContextStr,
}

#[repr(C)]
pub enum H3DCamera {
    PipeResI = 600,
    OutTexResI,
    OutBufIndexI,
    LeftPlaneF,
    RightPlaneF,
    BottomPlaneF,
    TopPlaneF,
    NearPlaneF,
    FarPlaneF,
    ViewportXI,
    ViewportYI,
    ViewportWidthI,
    ViewportHeightI,
    OrthoI,
    OccCullingI,
}

#[repr(C)]
pub enum H3DEmitter {
    MatResI = 700,
    PartEffResI,
    MaxCountI,
    RespawnCountI,
    DelayF,
    EmissionRateF,
    SpreadAngleF,
    ForceF3,
}

#[repr(C)]
pub enum H3DComputeNode {
    MatResI = 800,
    CompBufResI,
    AABBMinF,
    AABBMaxF,
    DrawTypeI,
    ElementsCountI,
}

#[repr(C)]
pub enum H3DModelUpdateFlags {
    Animation = 1,
    Geometry = 2,
}


extern "C" {
    #[link_name = "\u{1}_h3dGetVersionString"]
    pub fn h3dGetVersionString() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_h3dCheckExtension"]
    pub fn h3dCheckExtension(extensionName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetError"]
    pub fn h3dGetError() -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dInit"]
    pub fn h3dInit(deviceType: H3DRenderDevice) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dRelease"]
    pub fn h3dRelease();
}
extern "C" {
    #[link_name = "\u{1}_h3dCompute"]
    pub fn h3dCompute(
        materialRes: H3DRes,
        context: *const ::std::os::raw::c_char,
        groupX: ::std::os::raw::c_int,
        groupY: ::std::os::raw::c_int,
        groupZ: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dRender"]
    pub fn h3dRender(cameraNode: H3DNode);
}
extern "C" {
    #[link_name = "\u{1}_h3dFinalizeFrame"]
    pub fn h3dFinalizeFrame();
}
extern "C" {
    #[link_name = "\u{1}_h3dClear"]
    pub fn h3dClear();
}
extern "C" {
    #[link_name = "\u{1}_h3dGetMessage"]
    pub fn h3dGetMessage(
        level: *mut ::std::os::raw::c_int,
        time: *mut f32,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetOption"]
    pub fn h3dGetOption(param: H3DOptions) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetOption"]
    pub fn h3dSetOption(param: H3DOptions, value: f32) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetStat"]
    pub fn h3dGetStat(param: H3DStats, reset: bool) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetDeviceCapabilities"]
    pub fn h3dGetDeviceCapabilities(param: H3DDeviceCapabilities) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_h3dShowOverlays"]
    pub fn h3dShowOverlays(
        verts: *const f32,
        vertCount: ::std::os::raw::c_int,
        colR: f32,
        colG: f32,
        colB: f32,
        colA: f32,
        materialRes: H3DRes,
        flags: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dClearOverlays"]
    pub fn h3dClearOverlays();
}
extern "C" {
    #[link_name = "\u{1}_h3dGetResType"]
    pub fn h3dGetResType(res: H3DRes) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetResName"]
    pub fn h3dGetResName(res: H3DRes) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNextResource"]
    pub fn h3dGetNextResource(type_: ::std::os::raw::c_int, start: H3DRes) -> H3DRes;
}
extern "C" {
    #[link_name = "\u{1}_h3dFindResource"]
    pub fn h3dFindResource(
        type_: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    ) -> H3DRes;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddResource"]
    pub fn h3dAddResource(
        type_: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
        flags: ::std::os::raw::c_int,
    ) -> H3DRes;
}
extern "C" {
    #[link_name = "\u{1}_h3dCloneResource"]
    pub fn h3dCloneResource(sourceRes: H3DRes, name: *const ::std::os::raw::c_char) -> H3DRes;
}
extern "C" {
    #[link_name = "\u{1}_h3dRemoveResource"]
    pub fn h3dRemoveResource(res: H3DRes) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dIsResLoaded"]
    pub fn h3dIsResLoaded(res: H3DRes) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dLoadResource"]
    pub fn h3dLoadResource(
        res: H3DRes,
        data: *const ::std::os::raw::c_char,
        size: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dUnloadResource"]
    pub fn h3dUnloadResource(res: H3DRes);
}
extern "C" {
    #[link_name = "\u{1}_h3dGetResElemCount"]
    pub fn h3dGetResElemCount(res: H3DRes, elem: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dFindResElem"]
    pub fn h3dFindResElem(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetResParamI"]
    pub fn h3dGetResParamI(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        elemIdx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetResParamI"]
    pub fn h3dSetResParamI(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        elemIdx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetResParamF"]
    pub fn h3dGetResParamF(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        elemIdx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        compIdx: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetResParamF"]
    pub fn h3dSetResParamF(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        elemIdx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        compIdx: ::std::os::raw::c_int,
        value: f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetResParamStr"]
    pub fn h3dGetResParamStr(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        elemIdx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetResParamStr"]
    pub fn h3dSetResParamStr(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        elemIdx: ::std::os::raw::c_int,
        param: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dMapResStream"]
    pub fn h3dMapResStream(
        res: H3DRes,
        elem: ::std::os::raw::c_int,
        elemIdx: ::std::os::raw::c_int,
        stream: ::std::os::raw::c_int,
        read: bool,
        write: bool,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    #[link_name = "\u{1}_h3dUnmapResStream"]
    pub fn h3dUnmapResStream(res: H3DRes);
}
extern "C" {
    #[link_name = "\u{1}_h3dQueryUnloadedResource"]
    pub fn h3dQueryUnloadedResource(index: ::std::os::raw::c_int) -> H3DRes;
}
extern "C" {
    #[link_name = "\u{1}_h3dReleaseUnusedResources"]
    pub fn h3dReleaseUnusedResources();
}
extern "C" {
    #[link_name = "\u{1}_h3dCreateTexture"]
    pub fn h3dCreateTexture(
        name: *const ::std::os::raw::c_char,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
        fmt: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> H3DRes;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetShaderPreambles"]
    pub fn h3dSetShaderPreambles(
        vertPreamble: *const ::std::os::raw::c_char,
        fragPreamble: *const ::std::os::raw::c_char,
        geomPreamble: *const ::std::os::raw::c_char,
        tessControlPreamble: *const ::std::os::raw::c_char,
        tessEvalPreamble: *const ::std::os::raw::c_char,
        computePreamble: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dSetMaterialUniform"]
    pub fn h3dSetMaterialUniform(
        materialRes: H3DRes,
        name: *const ::std::os::raw::c_char,
        a: f32,
        b: f32,
        c: f32,
        d: f32,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dResizePipelineBuffers"]
    pub fn h3dResizePipelineBuffers(
        pipeRes: H3DRes,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetRenderTargetData"]
    pub fn h3dGetRenderTargetData(
        pipelineRes: H3DRes,
        targetName: *const ::std::os::raw::c_char,
        bufIndex: ::std::os::raw::c_int,
        width: *mut ::std::os::raw::c_int,
        height: *mut ::std::os::raw::c_int,
        compCount: *mut ::std::os::raw::c_int,
        dataBuffer: *mut ::std::os::raw::c_void,
        bufferSize: ::std::os::raw::c_int,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeType"]
    pub fn h3dGetNodeType(node: H3DNode) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeParent"]
    pub fn h3dGetNodeParent(node: H3DNode) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeParent"]
    pub fn h3dSetNodeParent(node: H3DNode, parent: H3DNode) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeChild"]
    pub fn h3dGetNodeChild(node: H3DNode, index: ::std::os::raw::c_int) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddNodes"]
    pub fn h3dAddNodes(parent: H3DNode, sceneGraphRes: H3DRes) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dRemoveNode"]
    pub fn h3dRemoveNode(node: H3DNode);
}
extern "C" {
    #[link_name = "\u{1}_h3dCheckNodeTransFlag"]
    pub fn h3dCheckNodeTransFlag(node: H3DNode, reset: bool) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeTransform"]
    pub fn h3dGetNodeTransform(
        node: H3DNode,
        tx: *mut f32,
        ty: *mut f32,
        tz: *mut f32,
        rx: *mut f32,
        ry: *mut f32,
        rz: *mut f32,
        sx: *mut f32,
        sy: *mut f32,
        sz: *mut f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeTransform"]
    pub fn h3dSetNodeTransform(
        node: H3DNode,
        tx: f32,
        ty: f32,
        tz: f32,
        rx: f32,
        ry: f32,
        rz: f32,
        sx: f32,
        sy: f32,
        sz: f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeTransMats"]
    pub fn h3dGetNodeTransMats(node: H3DNode, relMat: *mut *const f32, absMat: *mut *const f32);
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeTransMat"]
    pub fn h3dSetNodeTransMat(node: H3DNode, mat4x4: *const f32);
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeParamI"]
    pub fn h3dGetNodeParamI(node: H3DNode, param: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeParamI"]
    pub fn h3dSetNodeParamI(
        node: H3DNode,
        param: ::std::os::raw::c_int,
        value: ::std::os::raw::c_int,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeParamF"]
    pub fn h3dGetNodeParamF(
        node: H3DNode,
        param: ::std::os::raw::c_int,
        compIdx: ::std::os::raw::c_int,
    ) -> f32;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeParamF"]
    pub fn h3dSetNodeParamF(
        node: H3DNode,
        param: ::std::os::raw::c_int,
        compIdx: ::std::os::raw::c_int,
        value: f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeParamStr"]
    pub fn h3dGetNodeParamStr(
        node: H3DNode,
        param: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeParamStr"]
    pub fn h3dSetNodeParamStr(
        node: H3DNode,
        param: ::std::os::raw::c_int,
        value: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeFlags"]
    pub fn h3dGetNodeFlags(node: H3DNode) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeFlags"]
    pub fn h3dSetNodeFlags(node: H3DNode, flags: ::std::os::raw::c_int, recursive: bool);
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeAABB"]
    pub fn h3dGetNodeAABB(
        node: H3DNode,
        minX: *mut f32,
        minY: *mut f32,
        minZ: *mut f32,
        maxX: *mut f32,
        maxY: *mut f32,
        maxZ: *mut f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dFindNodes"]
    pub fn h3dFindNodes(
        startNode: H3DNode,
        name: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetNodeFindResult"]
    pub fn h3dGetNodeFindResult(index: ::std::os::raw::c_int) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetNodeUniforms"]
    pub fn h3dSetNodeUniforms(node: H3DNode, uniformData: *mut f32, count: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "\u{1}_h3dCastRay"]
    pub fn h3dCastRay(
        node: H3DNode,
        ox: f32,
        oy: f32,
        oz: f32,
        dx: f32,
        dy: f32,
        dz: f32,
        numNearest: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dGetCastRayResult"]
    pub fn h3dGetCastRayResult(
        index: ::std::os::raw::c_int,
        node: *mut H3DNode,
        distance: *mut f32,
        intersection: *mut f32,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dCheckNodeVisibility"]
    pub fn h3dCheckNodeVisibility(
        node: H3DNode,
        cameraNode: H3DNode,
        checkOcclusion: bool,
        calcLod: bool,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddGroupNode"]
    pub fn h3dAddGroupNode(parent: H3DNode, name: *const ::std::os::raw::c_char) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddModelNode"]
    pub fn h3dAddModelNode(
        parent: H3DNode,
        name: *const ::std::os::raw::c_char,
        geometryRes: H3DRes,
    ) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetupModelAnimStage"]
    pub fn h3dSetupModelAnimStage(
        modelNode: H3DNode,
        stage: ::std::os::raw::c_int,
        animationRes: H3DRes,
        layer: ::std::os::raw::c_int,
        startNode: *const ::std::os::raw::c_char,
        additive: bool,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetModelAnimParams"]
    pub fn h3dGetModelAnimParams(
        modelNode: H3DNode,
        stage: ::std::os::raw::c_int,
        time: *mut f32,
        weight: *mut f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dSetModelAnimParams"]
    pub fn h3dSetModelAnimParams(
        modelNode: H3DNode,
        stage: ::std::os::raw::c_int,
        time: f32,
        weight: f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dSetModelMorpher"]
    pub fn h3dSetModelMorpher(
        modelNode: H3DNode,
        target: *const ::std::os::raw::c_char,
        weight: f32,
    ) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dUpdateModel"]
    pub fn h3dUpdateModel(modelNode: H3DNode, flags: ::std::os::raw::c_int);
}
extern "C" {
    #[link_name = "\u{1}_h3dAddMeshNode"]
    pub fn h3dAddMeshNode(
        parent: H3DNode,
        name: *const ::std::os::raw::c_char,
        materialRes: H3DRes,
        batchStart: ::std::os::raw::c_int,
        batchCount: ::std::os::raw::c_int,
        vertRStart: ::std::os::raw::c_int,
        vertREnd: ::std::os::raw::c_int,
    ) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddJointNode"]
    pub fn h3dAddJointNode(
        parent: H3DNode,
        name: *const ::std::os::raw::c_char,
        jointIndex: ::std::os::raw::c_int,
    ) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddLightNode"]
    pub fn h3dAddLightNode(
        parent: H3DNode,
        name: *const ::std::os::raw::c_char,
        materialRes: H3DRes,
        lightingContext: *const ::std::os::raw::c_char,
        shadowContext: *const ::std::os::raw::c_char,
    ) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddCameraNode"]
    pub fn h3dAddCameraNode(
        parent: H3DNode,
        name: *const ::std::os::raw::c_char,
        pipelineRes: H3DRes,
    ) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dSetupCameraView"]
    pub fn h3dSetupCameraView(
        cameraNode: H3DNode,
        fov: f32,
        aspect: f32,
        nearDist: f32,
        farDist: f32,
    );
}
extern "C" {
    #[link_name = "\u{1}_h3dGetCameraProjMat"]
    pub fn h3dGetCameraProjMat(cameraNode: H3DNode, projMat: *mut f32);
}
extern "C" {
    #[link_name = "\u{1}_h3dSetCameraProjMat"]
    pub fn h3dSetCameraProjMat(cameraNode: H3DNode, projMat: *mut f32);
}
extern "C" {
    #[link_name = "\u{1}_h3dAddEmitterNode"]
    pub fn h3dAddEmitterNode(
        parent: H3DNode,
        name: *const ::std::os::raw::c_char,
        materialRes: H3DRes,
        particleEffectRes: H3DRes,
        maxParticleCount: ::std::os::raw::c_int,
        respawnCount: ::std::os::raw::c_int,
    ) -> H3DNode;
}
extern "C" {
    #[link_name = "\u{1}_h3dUpdateEmitter"]
    pub fn h3dUpdateEmitter(emitterNode: H3DNode, timeDelta: f32);
}
extern "C" {
    #[link_name = "\u{1}_h3dHasEmitterFinished"]
    pub fn h3dHasEmitterFinished(emitterNode: H3DNode) -> bool;
}
extern "C" {
    #[link_name = "\u{1}_h3dAddComputeNode"]
    pub fn h3dAddComputeNode(
        parent: H3DNode,
        name: *const ::std::os::raw::c_char,
        materialRes: H3DRes,
        compBufferRes: H3DRes,
        drawType: ::std::os::raw::c_int,
        elementsCount: ::std::os::raw::c_int,
    ) -> H3DNode;
}
