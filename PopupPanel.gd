extends PopupPanel


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var _db = null
var _trade = null

func setup(db, trade): 
	_db = db
	_trade = trade

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass


func _on_Accept_pressed():
	var message = {
		'origin': _trade['seller'],
		'state_change': {'Start': {}},
		'request': _trade['request']
	}
	_db.try_trade_handle(message)
	


func _on_Reject_pressed():
	var message = {
		'origin': _trade['seller'],
		'state_change': {'Rejected': {}},
		'request': _trade['request']
	}
	_db.try_trade_handle(message)
