extends Node2D

signal on_take(item)

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var _parent_entity = null
var _inventory_entities = []
var _db = null

onready var itemlist = get_node("ItemList")
onready var button = get_node("Button")

func setup(db, entity):
	_parent_entity = entity
	_db = db

func update_list(entities, names):
	_inventory_entities = entities
	itemlist.clear()
	for name in names: 
		itemlist.add_item(name)

func _process(_delta):
	if _db == null: return
	var inventory: Array = _db.get_inventory(_parent_entity)
	
	var entities = []
	var names = []
	var new = inventory.size() != _inventory_entities.size()
	for item in inventory:
		new = new || not(item['entity'] in _inventory_entities)
		entities.push_back(item['entity'])
		names.push_back(item['name'])	
	
	if new:
		update_list(entities, names)

func _on_Button_pressed():
	if itemlist.is_anything_selected(): 
		var index = itemlist.get_selected_items()[0]
		emit_signal("on_take", _inventory_entities[index])
		
