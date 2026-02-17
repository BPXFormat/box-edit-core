local Packager = require "bp3d.packager"
local build = require "bp3d.util.build"
local context = require "bp3d.util.context"
local artifact = require "bp3d.util.artifact"

local Dist = Class(Packager)

function Dist:init(args)
    Packager.init(self, args)
    self.bpxedit = {}
end

function Dist:packageTarget(ctx, artifacts)
    for _, v in ipairs(artifacts) do
        if v:name() == "BPXEdit.framework" then
            table.insert(self.bpxedit, v:path())
        end
    end
end

function Dist:package()
    local distFolder = self.context.path:join("target/dist")
    build.clean(distFolder)

    -- Include the core framework if it exists
    local coreFramework = self.context.path:join("core/target/BPXEditCore.xcframework")
    if bp3d.files.exists(coreFramework) then
        print("Copying BPXEditCore.xcframework...")
        build.run("cp", { "-R", coreFramework, distFolder:join("BPXEditCore.xcframework") })
    end

    -- Generate the BPXEdit XC framework if needed
    if #self.bpxedit > 0 then
        print("Generating BPXEdit.xcframework...")
        local args = { "xcodebuild", "-create-xcframework" }
        for _, path in ipairs(self.bpxedit) do
            table.insert(args, "-framework")
            table.insert(args, path)
        end
        table.insert(args, "-output")
        table.insert(args, distFolder:join("BPXEdit.xcframework"))
        build.run("xcrun", args)
    end
end

return Dist
