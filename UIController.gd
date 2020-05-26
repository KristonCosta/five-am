extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var _db = null

# Called when the node enters the scene tree for the first time.
func _ready():
	_db = get_node("../LogicController")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
	

func _on_LogicController_map_loaded(tiles, width):
	var map_controller = get_node("ViewportContainer/Viewport/MapController")
	map_controller.load_map(tiles, width)


func _on_LogicController_sync_entities(entities):
	pass # Replace with function body.


func _on_EntityController_on_entity_click(entity):
	var label = get_node("Panel/Label")
	label.text = _db.get_name(entity)
