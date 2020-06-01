extends Node2D

onready var graphic_tileset: TileSet = preload("res://graphic_tileset.tres")
onready var ascii_texture: Texture = preload("res://test.png")

var CHARS = """╦╩═╬╧╨╤╥╙╘╒╓╫╪┘╠┌█▄▌▐▀αßΓπΣσµτΦδ∞φ╟╚╔║╗╝╣╢╖
*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQ⌠⌡≥
RSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxy÷≈
z{|}~⌂ÇüéâäàåçêëèïîìÄÅÉæÆôöòûùÿÖÜ¢£¥₧ƒáí°∙
óúñÑªº¿⌐¬½¼¡«»░▒▓│┤╡╕╜╛┐└┴┬├─┼╞·√±≤ⁿε∩≡ΘΩ
\"☺☻♥♦♣♠•◘○◙♂♀♪♫☼►◄↕‼¶§▬↨↑↓→←∟↔▲▼!#$%&'()²■"""

var _atlas = {}

# Called when the node enters the scene tree for the first time.
func _ready():
	var x = 0
	var y = 0
	for c in CHARS:
		if c == '\n':
			y += 1
			x = 0
		else:
			_atlas[c] = Rect2(x * 20.0, y * 40.0, 20.0, 40.0)
			x += 1

var ascii_map = {
	'1': "cherries",
	'2': "cabbage",
	'3': "orange",
	'4': "cabbage",
}

class TextureBundle:
	var region: Rect2
	var texture: Texture
	
	func get_texture():
		return texture
	
	func get_region(): 
		return region 

func get_bundle(c): 
	if ascii_map.has(c):
		return load_graphic(c)
	else:
		return load_ascii(c)

func load_ascii(c):
	var region_rect = _atlas.get(c)
	var bundle: TextureBundle = TextureBundle.new()
	bundle.region = region_rect 
	bundle.texture = ascii_texture
	return bundle

func load_graphic(c):
	var tile_id = graphic_tileset.find_tile_by_name(ascii_map.get(c))
	var texture = graphic_tileset.tile_get_texture(tile_id)
	var region_rect = Rect2(0, 0, texture.get_width(), texture.get_height())
	
	var bundle: TextureBundle = TextureBundle.new()
	bundle.region = region_rect 
	bundle.texture = texture
	return bundle
