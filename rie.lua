------------------------------------------
-- setup

local g = golly()
local rie = "golly/FlipIfElse"

------------------------------------------
-- helpers

local logs = ""
function log(thing)
	logs = logs..tostring(thing)..' '
end

local function show(s)
	g.show(s)
	g.update()
end

------------------------------------------
-- functions

local function rom_location()
	g.run(1)
	local tx, ty = table.unpack(g.getrect())
	g.reset()
	return tx + 4, ty + 5
end

------------------------------------------
-- begin

show "Compiling..."
os.execute("cargo run "..rie.." --clip")

show "Finding Pattern..." 
local x, y = rom_location()
show "Pattern found. Assuming it is a turing machine without question."

show "Pasting..."
g.paste(x, y, "or")
show "Pasted. Run pattern at 2^8 or 8^3."

-- show(logs)
