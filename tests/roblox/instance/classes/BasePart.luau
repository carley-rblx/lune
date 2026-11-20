local roblox = require("@lune/roblox")
local CFrame = roblox.CFrame
local Instance = roblox.Instance
local Vector3 = roblox.Vector3

local function assertFuzzyEq(actual, expected)
	assert(
		(actual - expected).Magnitude < 1e-4,
		string.format("Expected %s, got %s", tostring(expected), tostring(actual))
	)
end

-- Position, Orientation and Rotation should be readable on a new part

local part = Instance.new("Part") :: any
assertFuzzyEq(part.Position, Vector3.zero)
assertFuzzyEq(part.Orientation, Vector3.zero)
assertFuzzyEq(part.Rotation, Vector3.zero)

-- They should read from the CFrame

part.CFrame = CFrame.new(1, 2, 3) * CFrame.fromOrientation(math.rad(10), math.rad(20), math.rad(30))
assertFuzzyEq(part.Position, Vector3.new(1, 2, 3))
assertFuzzyEq(part.Orientation, Vector3.new(10, 20, 30))
local rx, ry, rz = part.CFrame:ToEulerAnglesXYZ()
assertFuzzyEq(part.Rotation, Vector3.new(math.deg(rx), math.deg(ry), math.deg(rz)))

-- Writing Position should move the part and keep its rotation

part.Position = Vector3.new(7, 8, 9)
assertFuzzyEq(part.CFrame.Position, Vector3.new(7, 8, 9))
assertFuzzyEq(part.Orientation, Vector3.new(10, 20, 30))

-- Writing Orientation or Rotation should turn the part and keep its position

part.Orientation = Vector3.new(0, 90, 0)
assertFuzzyEq(part.CFrame.Position, Vector3.new(7, 8, 9))
assertFuzzyEq(part.CFrame.LookVector, Vector3.new(-1, 0, 0))

part.Rotation = Vector3.new(90, 0, 0)
assertFuzzyEq(part.CFrame.Position, Vector3.new(7, 8, 9))
assertFuzzyEq(part.CFrame.UpVector, Vector3.new(0, 0, 1))

-- A write should be kept through serialization, since only the CFrame is saved

part.Position = Vector3.new(4, 5, 6)
local loaded = roblox.deserializeModel(roblox.serializeModel({ part }))[1] :: any
assertFuzzyEq(loaded.Position, Vector3.new(4, 5, 6))
assertFuzzyEq(loaded.CFrame.Position, Vector3.new(4, 5, 6))

-- This should hold for every kind of BasePart

local wedge = Instance.new("WedgePart") :: any
wedge.Position = Vector3.new(0, 10, 0)
assertFuzzyEq(wedge.CFrame.Position, Vector3.new(0, 10, 0))

-- Other classes with a Position property should be left as they were

local frame = Instance.new("Frame") :: any
frame.Position = roblox.UDim2.fromScale(0.5, 0.5)
assert(frame.Position == roblox.UDim2.fromScale(0.5, 0.5))
