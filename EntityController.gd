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
var entities = {}

onready var entity_scene = preload("res://entity.tscn")

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

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass

func _on_LogicController_created_entities(entities):
	var db = get_node("../../../../LogicController")
	for entity in entities: 
		var scene = entity_scene.instance()
		scene.setup(db, ATLAS, entity)
		add_child(scene)

func _on_LogicController_deleted_entities(entities):
	print(entities)
