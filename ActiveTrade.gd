extends PopupPanel


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var _db = null
var _trade = null
var _texture_loader = null
onready var target_entity = get_node("Panel/Target")
onready var buyer_entity = get_node("Panel/Buyer")
onready var seller_entity = get_node("Panel/Seller")
onready var current_offer = get_node("Panel/CurrentOffer")
onready var spin_box = get_node("Panel/SpinBox")

var _next_responder = null
var _next_offer = null

var _current_offer = null

func setup(db, texture_loader, trade): 
	_db = db
	_trade = trade
	_texture_loader = texture_loader 
	_current_offer = null
	_next_responder = _trade['buyer']
	_next_offer = 'Offer'
	if _trade['buyer'] == _trade['last_response']: 
		_next_responder = _trade['seller']
		_next_offer = 'CounterOffer'
	
	if _trade['trade_state'].has('Offer'):
		_current_offer = _trade['trade_state']['Offer']
		
	if _trade['trade_state'].has('CounterOffer'):
		_current_offer = _trade['trade_state']['CounterOffer']
	
	if _current_offer != null:
		current_offer.text = str(_current_offer)
		spin_box.value = _current_offer
		
	target_entity.setup(_db, _texture_loader, _trade['target'])
	buyer_entity.setup(_db, _texture_loader, _trade['buyer'])
	seller_entity.setup(_db, _texture_loader, _trade['seller'])

# Called when the node enters the scene tree for the first time.
func _ready():
	if _current_offer != null:
		var offer = get_node("Panel/CurrentOffer")
		offer.text = _current_offer


func _on_ActiveOffer_pressed():
	var offer = get_node("Panel/SpinBox")
	print("Entering")
	var message = {
		'origin': _next_responder,
		'state_change': {_next_offer: int(offer.value)},
		'request': _trade['request']
	}
	print(_trade)
	print(message)
	_db.try_trade_handle(message)


func _on_ActiveReject_pressed():
	var message = {
		'origin': _trade['seller'],
		'state_change': {'Rejected': {}},
		'request': _trade['request']
	}
	_db.try_trade_handle(message)


func _on_ActiveAccept_pressed():
	var message = {
		'origin': _next_responder,
		'state_change': {'Accepted': {}},
		'request': _trade['request']
	}
	_db.try_trade_handle(message)
