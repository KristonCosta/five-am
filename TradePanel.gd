extends Node2D


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

var pending_trade = null
var _trade = null
var _db = null 
var _texture_loader = null

onready var pending_popup = get_node("PendingTrade")
onready var active_popup = get_node("ActiveTrade")
onready var accepted_popup = get_node("AcceptedTrade")
onready var rejected_popup = get_node("RejectedTrade")


func setup(db, texture_loader):
	_db = db
	_texture_loader = texture_loader

func bind(trade):
	pending_trade = trade 
	
	#	target_entity.setup(_db, _texture_loader, _trade['target'])
	#	buyer_entity.setup(_db, _texture_loader, _trade['buyer'])
	#	seller_entity.setup(_db, _texture_loader, _trade['seller'])
	#
	#
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if pending_trade != null: 
		_trade = pending_trade
		print(_trade)
		if _trade['trade_state'].has('Pending'):
			pending_popup.setup(_db, _trade)
			pending_popup.popup_centered(Vector2(400, 120))
		if _trade['trade_state'].has('Start') || _trade['trade_state'].has('Offer') || _trade['trade_state'].has('CounterOffer'):
			active_popup.setup(_db, _texture_loader, _trade)
			active_popup.popup_centered(Vector2())
		if _trade['trade_state'].has('Rejected'):
			rejected_popup.popup_centered(Vector2())
		if _trade['trade_state'].has('Final'):
			accepted_popup.setup(_trade['trade_state']['Final'])
			accepted_popup.popup_centered(Vector2())
		pending_trade = null 
		
