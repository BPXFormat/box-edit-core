local Builder = require "bp3d.builder"
local BPXEdit = require "bpx-edit-core.BPXEdit"

local BPXEditCore = Class(Builder)

BPXEditCore.name = "bpx-edit-core"
BPXEditCore.version = "1.0.0"

local COMPONENTS = { BPXEdit }

function BPXEditCore:configure(ctx)
    for _, v in ipairs(COMPONENTS) do
        local flag = true
        for _, t in ipairs(ctx.targets) do
            ctx.target = t
            if not v.isAvailable(ctx) then
                flag = false
            end
        end
        if flag then
            v:configure(ctx)
        end
    end
end

function BPXEditCore:build(ctx)
    for _, v in ipairs(COMPONENTS) do
        if v.isAvailable(ctx) then
            v:build(ctx)
        end
    end
end

function BPXEditCore:prePackage(ctx)
    local artifacts = bp3d.build.List.new()
    for _, v in ipairs(COMPONENTS) do
        if v.isAvailable(ctx) then
            v:prePackage(ctx, artifacts)
        end
    end
    return artifacts
end

return BPXEditCore
