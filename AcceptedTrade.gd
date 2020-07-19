extends PopupPanel


# Declare member variables here. Examples:
# var a = 2
# var b = "text"
onready var result = get_node("Panel/Request")

func setup(final_price): 
	result.text = "Final price " + str(final_price)

# Called when the node enters the scene tree for the first time.
func _ready():
	pass


func _on_Ok_pressed():
	self.hide() 
