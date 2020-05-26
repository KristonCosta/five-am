extends Node2D

signal on_entity_click(entity)

var _db = null 
var _entity = null 
var _atlas = null
var _initialized = false

func set_grid_pos(pos: Vector2):
	position = Vector2(pos.x * 20.0 + 10.0, pos.y * 40.0 + 20.0)

func setup(db, atlas, entity):
	_db = db
	_entity = entity
	_atlas = atlas
	var c = db.get_renderable(entity)
	var region: Vector2 = atlas.get(c)
	var sprite = get_node("Sprite")
	sprite.region_enabled = true 
	sprite.region_rect = Rect2(region.x * 20.0, region.y * 40.0, 20.0, 40.0)
	var clickable = get_node("Sprite/Area2D")
	clickable.input_pickable = true
	set_grid_pos(db.get_position(entity))
	_initialized = true 

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if not _initialized: return
	set_grid_pos(_db.get_position(_entity))
	
func _on_Area2D_input_event(viewport, event, shape_idx):
	print("Clicked!")
	if event is InputEventMouseButton:
		if event.is_action_pressed("click"):
			emit_signal("on_entity_click", _entity)
	
