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


func _on_ClientController_clicked_on_entity(entity):
	var controller = get_node("../PlayerCamera/ClientController")
	var label = get_node("Panel/Label")
	var itemlist = get_node("Panel/ItemList")
	label.text = controller.get_name(entity)
	itemlist.clear()
	for item in controller.get_inventory(entity):
		itemlist.add_item(item["name"])
	
