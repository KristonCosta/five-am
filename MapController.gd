extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var CHARS = """╦╩═╬╧╨╤╥╙╘╒╓╫╪┘╠┌█▄▌▐▀αßΓπΣσµτΦδ∞φ╟╚╔║╗╝╣╢╖
*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQ⌠⌡≥
RSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxy÷≈
z{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáí°∙
óúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╕╜╛┐└┴┬├─┼╞·√±≤ⁿε∩≡ΘΩ
\"☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼!#$%&'()²■"""

var ATLAS = {}

var tile_callbacks = {
	'#': funcref(self, "load_wall"),
	'.': funcref(self, "load_floor"),
}

# Called when the node enters the scene tree for the first time.
func _ready():
	var x = 0
	var y = 0
	for c in CHARS:
		if c == '\n':
			y += 1
			x = 0
		else:
			ATLAS[c] = Vector2(x, y)
			x += 1
			
func load_wall(x, y, map_matrix, map: TileMap):
	var height = map_matrix.size()
	var width = map_matrix[0].size()
	var tile = map_matrix[y][x]
	var tileset = map.tile_set
	
	var tile_name = 'cherries'
	var found = false
	if y + 1 < height:
		var bottom_tile = map_matrix[y+1][x]
		if bottom_tile == '.':
			tile_name = 'Brickwallmiddle'
			found = true
	if y + 1 >= height: 
		#ignore the last row I think
		return
	if x + 1 < width && !found:
		var right_tile = map_matrix[y][x + 1]
		if right_tile == '.' || right_tile == '#':
			tile_name = 'Outerwallleft'
			found = true
	if x - 1 >= 0 && !found:
		var left_tile = map_matrix[y][x - 1]
		if left_tile == '.' || left_tile == '#':
			tile_name = 'Outerwallright'
			found = true
	var tile_id = tileset.find_tile_by_name(tile_name)
	var texture = tileset.tile_get_texture(tile_id)
	map.set_cell(x, y, tile_id)

func load_floor(x, y, map_matrix, map: TileMap):
	var tile = map_matrix[y][x]
	var tileset = map.tile_set
	var tile_id = tileset.find_tile_by_name('Darkwoodmiddle')
	var texture = tileset.tile_get_texture(tile_id)
	map.set_cell(x, y, tile_id)

func load_map(tiles, width): 
	var graphic_map = get_node("GraphicTileMap")
	var ascii_map = get_node("AsciiTileMap")
	ascii_map.clear()
	graphic_map.clear()
	var tileset = ascii_map.tile_set
	tileset.tile_set_region(0, Rect2(0.0, 0.0, 860.0, 240.0))
	tileset.autotile_set_size(0, Vector2(20.0, 40.0))
	
	var x = 0
	var y = 0
	
	var map_matrix=[[]]
	for tile in tiles:
		map_matrix[y].append(tile)
		x += 1 
		if x >= width:
			map_matrix.append([])
			x = 0
			y += 1
	if x == 0: 
		map_matrix.pop_back()
		y -= 1
	var height = y + 1
	
	for x in range(width):
		for y in range(height):
			var tile = map_matrix[y][x]
			var region
			if tile_callbacks.has(tile): 
				tile_callbacks[tile].call_func(x, y, map_matrix, graphic_map)	
				region = ATLAS.get(tile)
				ascii_map.set_cell(x, y, 0, false, false, false, region)
			else:
				region = ATLAS.get(tile)
				ascii_map.set_cell(x, y, 0, false, false, false, region)

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
