extends Node2D

signal on_entity_click(entity)

var _db = null 
var _entity = null 
var _texture_loader = null

onready var display_case_scene = preload("res://DisplayCase.tscn")
onready var graphic_tileset: TileSet = preload("res://graphic_tileset.tres")

onready var sprite = get_node("Sprite")
onready var clickable = get_node("Sprite/Area2D")

func setup(db, texture_loader, entity):
	_db = db
	_entity = entity
	_texture_loader = texture_loader
	reload()
 
func reload():
	if _entity == null: return
	var c = _db.get_renderable(_entity)
	var bundle = _texture_loader.get_bundle(c)
	sprite.texture = bundle.get_texture()
	sprite.region_enabled = true 
	sprite.region_rect = bundle.get_region()
	sprite.scale = Vector2(32.0 / bundle.get_region().size.x, 32.0 / bundle.get_region().size.y)
	
	clickable.input_pickable = true
	
	if _db.is_display_case(_entity): 
		var scene = display_case_scene.instance()
		scene.setup(_db, _texture_loader, _entity)
		add_child(scene)

	
func _on_Area2D_input_event(viewport, event, shape_idx):
	if event is InputEventMouseButton:
		if event.is_action_pressed("click"):
			emit_signal("on_entity_click", _entity)
