extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


func setup(db, atlas, entity):
	var c = db.get_renderable(entity)
	var region: Vector2 = atlas.get(c)
	var sprite = get_node("Sprite")
	sprite.region_enabled = true 
	sprite.region_rect = Rect2(region.x * 20.0, region.y * 40.0, 20.0, 40.0)
	
	var pos: Vector2 = db.get_position(entity)
	position = Vector2(pos.x * 20.0, pos.y * 40.0)
	print(db.get_name(entity))

# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
