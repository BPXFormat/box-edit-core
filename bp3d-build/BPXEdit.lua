local build = require "bp3d.util.build"
local context = require "bp3d.util.context"

local BPXEdit = Class()

function BPXEdit:configure(ctx)
    build.runBP3D("core", "package", ctx, { "-p", "framework" })
end

function BPXEdit.isAvailable(ctx)
    return bp3d.util.utf8.contains(ctx.target, "apple")
end

local function build1(ctx, args)
    bp3d.util.table.concat(args, { "VALID_ARCHS=arm64", "-scheme", "BPXEdit" })
    if bp3d.util.utf8.contains(ctx.target, "darwin") then
        bp3d.util.table.concat(args, { "-destination", "generic/platform=macOS" })
    elseif bp3d.util.utf8.contains(ctx.target, "ios") then
        bp3d.util.table.concat(args, { "-destination", "generic/platform=iOS" })
    end
    bp3d.util.table.concat(args, { "clean", "build" })
    build.run("xcodebuild", args, {
        workdir = ctx.path:join("BPXEdit")
    })
end

function BPXEdit:build(ctx)
    build1(ctx, {})
end

function BPXEdit:prePackage(ctx, artifacts)
    local path = context.getTargetPath(ctx)
    build1(ctx, { "SYMROOT=" .. bp3d.build.files.toString(path) })
    if bp3d.util.utf8.contains(ctx.target, "darwin") then
        artifacts:add(bp3d.build.Artifact.resource(path:join("Debug/BPXEdit.framework"), "BPXEdit.framework"))
    elseif bp3d.util.utf8.contains(ctx.target, "ios") then
        artifacts:add(bp3d.build.Artifact.resource(path:join("Debug-iphoneos/BPXEdit.framework"), "BPXEdit.framework"))
    end
end

return BPXEdit
