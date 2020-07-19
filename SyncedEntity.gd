extends Node2D

signal on_entity_click(entity)

var _db = null 
var _entity = null 
var _texture_loader = null

func set_grid_pos(pos: Vector2):
	position = Vector2(pos.x * 32.0 + 16.0, pos.y * 32.0 + 16.0)

func setup(db, texture_loader, entity):
	_db = db
	_entity = entity
	_texture_loader = texture_loader

func _ready():
	var entity = get_node("Entity")
	entity.setup(_db, _texture_loader, _entity)

func _process(delta):
	if _entity == null: return
	set_grid_pos(_db.get_position(_entity))

func _on_Entity_on_entity_click(entity):
	emit_signal("on_entity_click", _entity)
