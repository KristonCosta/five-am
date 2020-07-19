extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var _db = null

onready var givetake_scene = preload("res://GiveTakePanel.tscn")
onready var trade_scene = preload("res://TradePanel.tscn")
onready var texture_loader = get_node("TextureLoader")
var active_panel = null 

# Called when the node enters the scene tree for the first time.
func _ready():
	_db = get_node("../LogicController")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
	
func _on_LogicController_map_loaded(tiles, width):
	print("Handling")
	var map_controller = get_node("MapController")
	map_controller.load_map(tiles, width)

func close_panel(scene):
	get_node("HUD").remove_child(scene)
	active_panel = null

func _on_EntityController_on_entity_click(entity):
	if _db.is_display_case(entity): 
		var scene = givetake_scene.instance()
		var player = _db.get_player()
		scene.setup(_db, player, entity)
		if active_panel != null: 
			close_panel(active_panel)
		active_panel = scene
		get_node("HUD").add_child(scene)
		scene.connect("done", self, "close_panel", [active_panel])
		


func _on_LogicController_trade_event(trade):
	var scene = trade_scene.instance()
	scene.setup(_db, texture_loader)
	if active_panel != null: 
		close_panel(active_panel)
	scene.bind(trade)
	active_panel = scene
	get_node("HUD").add_child(scene)
	
