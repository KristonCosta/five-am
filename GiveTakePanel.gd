extends Panel

signal done()

var _db = null 
var _player = null 
var _other = null 

onready var player = get_node("Player")
onready var other = get_node("Other")

# Called when the node enters the scene tree for the first time.
func _ready():
	player.setup(_db, _player)
	other.setup(_db, _other)
	
func setup(db, player, other): 
	_db = db
	_player = player
	_other = other

func _on_Other_on_take(item):
	_db.try_take(_other)
	emit_signal("done")
	
func _on_Player_on_give(item):
	_db.try_put(_other, item)
	emit_signal("done")
