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

func load_map(tiles, width): 
	var map = get_node("TileMap")
	map.clear()
	var tileset = map.tile_set
	tileset.tile_set_region(0, Rect2(0.0, 0.0, 860.0, 240.0))
	tileset.autotile_set_size(0, Vector2(20.0, 40.0))
	var x = 0
	var y = 0
	for tile in tiles:
		var region = ATLAS.get(tile)
		map.set_cell(x, y, 0, false, false, false, region)
		x += 1 
		if x >= width:
			x = 0
			y += 1

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
