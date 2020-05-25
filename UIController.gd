extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var pending_event = null

# Called when the node enters the scene tree for the first time.
func _ready():
	var node = get_node("../PlayerCamera/ClientController")
	print(node.ping())
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass



func _on_ClientController_clicked_on_entity(data):
	var node = get_node("../PlayerCamera/ClientController")
	print(node.get_name(data))
