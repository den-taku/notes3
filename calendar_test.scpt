tell application "Calendar"
	tell calendar "Test"
		make new event at end with properties {summary:"Test", start date:(current date), end date:((current date) + (1 * hours))}
	end tell
end tell
