extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var _db = null

onready var givetake_scene = preload("res://GiveTakePanel.tscn")
var givetake = null 

# Called when the node enters the scene tree for the first time.
func _ready():
	_db = get_node("../LogicController")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
	
func _on_LogicController_map_loaded(tiles, width):
	var map_controller = get_node("MapController")
	map_controller.load_map(tiles, width)


func _on_LogicController_sync_entities(entities):
	pass # Replace with function body.


func close_panel(scene):
	get_node("HUD").remove_child(scene)
	givetake = null

func _on_EntityController_on_entity_click(entity):
	if _db.is_display_case(entity): 
		var scene = givetake_scene.instance()
		var player = _db.get_player()
		scene.setup(_db, player, entity)
		if givetake != null: 
			close_panel(givetake)
		givetake = scene
		get_node("HUD").add_child(scene)
		scene.connect("done", self, "close_panel", [givetake])
		
	var label = get_node("HUD/Panel/Label")
	label.text = _db.get_name(entity)
