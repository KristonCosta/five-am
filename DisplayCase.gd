extends Node2D

var _db = null 
var _parent_entity = null 
var _texture_loader = null
var _initialized = false
var _entity = null 

onready var _sprite = get_node("Sprite")

func update_entity(entity): 
	_entity = entity
	var letter = _db.get_renderable(entity)
	var bundle = _texture_loader.get_bundle(letter)
	_sprite.region_rect = bundle.get_region()
	_sprite.texture = bundle.get_texture()
	_sprite.scale = Vector2(20.0 / bundle.get_region().size.x, 15.0 / bundle.get_region().size.y)
	_sprite.position = Vector2(0.0, -5.0)
	
func clear_entity():
	_sprite.region_rect = Rect2(0.0, 0.0, 0.0, 0.0)

func _ready(): 
	_sprite.region_enabled = true 	

func setup(db, texture_loader, entity):
	_db = db
	_parent_entity = entity
	_texture_loader = texture_loader

func _process(_delta):
	var inventory: Array = _db.get_inventory(_parent_entity)
	if inventory.empty():
		if _entity != null:
			clear_entity()
	elif inventory[0]['entity'] != _entity:
		update_entity(inventory[0]['entity'])
	
	
