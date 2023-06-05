# bevy_ressys

Annotation macro for writing systems that return results for logging for [Bevy](https://github.com/bevyengine/bevy).



This is an implementation of a potential fix for https://github.com/bevyengine/bevy/issues/8638, where piping systems that return errors to the logging util systems reports the location of the error as the util system and not the system that generated the error.


