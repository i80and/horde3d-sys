extern crate cc;

use std::path::PathBuf;

fn main() {
    let mut cxx = cc::Build::new();
    cxx.cpp(true);
    cxx.include("vendor/Horde3D/Source/Horde3DEngine");
    cxx.include("vendor/Horde3D/Source/Shared");
    cxx.include("vendor/Extensions");
    cxx.warnings(false);

    cxx.define("PLATFORM_MAC", "1");

    let files_engine = &[
        "egAnimatables.cpp",
        "egAnimation.cpp",
        "egCamera.cpp",
        "egCom.cpp",
        "egComputeNode.cpp",
        "egComputeBuffer.cpp",
        "egExtensions.cpp",
        "egGeometry.cpp",
        "egLight.cpp",
        "egMain.cpp",
        "egMaterial.cpp",
        "egModel.cpp",
        "egModules.cpp",
        "egParticle.cpp",
        "egPipeline.cpp",
        "egPrimitives.cpp",
        "egRendererBaseGL2.cpp",
        "egRendererBaseGL4.cpp",
        "egRenderer.cpp",
        "egResource.cpp",
        "egScene.cpp",
        "egSceneGraphRes.cpp",
        "egShader.cpp",
        "egTexture.cpp",
        "utImage.cpp",
        "utOpenGL.cpp",
    ];

    let files_terrain = &["extension.cpp", "terrain.cpp"];

    let files_external_texture = &["extension.cpp", "egTextureEx.cpp"];

    for file in files_engine {
        cxx.file(PathBuf::from("vendor/Horde3D/Source/Horde3DEngine").join(file));
    }

    for file in files_terrain {
        cxx.file(PathBuf::from("vendor/Extensions/Terrain/Source").join(file));
    }

    for file in files_external_texture {
        cxx.file(PathBuf::from("vendor/Extensions/ExternalTexture/Source").join(file));
    }

    cxx.compile("libhorde3d.a");
}
