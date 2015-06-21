initSidebarItems({"type":[["Result","A universal result type used as return for all calls."]],"struct":[["AchievementMethods","A builder providing access to all methods supported on *achievement* resources. It is not used directly, but through the `GamesManagement` hub."],["AchievementResetAllCall","Resets all achievements for the currently authenticated player for your application. This method is only accessible to whitelisted tester accounts for your application."],["AchievementResetAllForAllPlayerCall","Resets all draft achievements for all players. This method is only available to user accounts for your developer console."],["AchievementResetAllResponse","This is a JSON template for achievement reset all response."],["AchievementResetCall","Resets the achievement with the given ID for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application."],["AchievementResetForAllPlayerCall","Resets the achievement with the given ID for all players. This method is only available to user accounts for your developer console. Only draft achievements can be reset."],["AchievementResetMultipleForAllPlayerCall","Resets achievements with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft achievements may be reset."],["AchievementResetMultipleForAllRequest","This is a JSON template for multiple achievements reset all request."],["AchievementResetResponse","This is a JSON template for an achievement reset response."],["ApplicationListHiddenCall","Get the list of players hidden from the given application. This method is only available to user accounts for your developer console."],["ApplicationMethods","A builder providing access to all methods supported on *application* resources. It is not used directly, but through the `GamesManagement` hub."],["DefaultDelegate","A delegate with a conservative default implementation, which is used if no other delegate is set."],["ErrorResponse","A utility to represent detailed errors we might see in case there are BadRequests. The latter happen if the sent parameters or request structures are unsound"],["EventMethods","A builder providing access to all methods supported on *event* resources. It is not used directly, but through the `GamesManagement` hub."],["EventResetAllCall","Resets all player progress on all events for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application. All quests for this player will also be reset."],["EventResetAllForAllPlayerCall","Resets all draft events for all players. This method is only available to user accounts for your developer console. All quests that use any of these events will also be reset."],["EventResetCall","Resets all player progress on the event with the given ID for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application. All quests for this player that use the event will also be reset."],["EventResetForAllPlayerCall","Resets the event with the given ID for all players. This method is only available to user accounts for your developer console. Only draft events can be reset. All quests that use the event will also be reset."],["EventResetMultipleForAllPlayerCall","Resets events with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft events may be reset. All quests that use any of the events will also be reset."],["EventsResetMultipleForAllRequest","This is a JSON template for multiple events reset all request."],["GamesManagement","Central instance to access all GamesManagement related resource activities"],["GamesPlayedResource","This is a JSON template for metadata about a player playing a game with the currently authenticated user."],["GamesPlayerExperienceInfoResource","This is a JSON template for 1P/3P metadata about the player's experience."],["GamesPlayerLevelResource","This is a JSON template for 1P/3P metadata about a user's level."],["HiddenPlayer","This is a JSON template for the HiddenPlayer resource."],["HiddenPlayerList","This is a JSON template for a list of hidden players."],["MethodInfo","Contains information about an API request."],["MultiPartReader","Provides a `Read` interface that converts multiple parts into the protocol identified by RFC2387. **Note**: This implementation is just as rich as it needs to be to perform uploads to google APIs, and might not be a fully-featured implementation."],["Player","This is a JSON template for a Player resource."],["PlayerHideCall","Hide the given player's leaderboard scores from the given application. This method is only available to user accounts for your developer console."],["PlayerMethods","A builder providing access to all methods supported on *player* resources. It is not used directly, but through the `GamesManagement` hub."],["PlayerName","An object representation of the individual components of the player's name. For some players, these fields may not be present."],["PlayerScoreResetAllResponse","This is a JSON template for a list of leaderboard reset resources."],["PlayerScoreResetResponse","This is a JSON template for a list of reset leaderboard entry resources."],["PlayerUnhideCall","Unhide the given player's leaderboard scores from the given application. This method is only available to user accounts for your developer console."],["QuestMethods","A builder providing access to all methods supported on *quest* resources. It is not used directly, but through the `GamesManagement` hub."],["QuestResetAllCall","Resets all player progress on all quests for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application."],["QuestResetAllForAllPlayerCall","Resets all draft quests for all players. This method is only available to user accounts for your developer console."],["QuestResetCall","Resets all player progress on the quest with the given ID for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application."],["QuestResetForAllPlayerCall","Resets all player progress on the quest with the given ID for all players. This method is only available to user accounts for your developer console. Only draft quests can be reset."],["QuestResetMultipleForAllPlayerCall","Resets quests with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft quests may be reset."],["QuestsResetMultipleForAllRequest","This is a JSON template for multiple quests reset all request."],["RoomMethods","A builder providing access to all methods supported on *room* resources. It is not used directly, but through the `GamesManagement` hub."],["RoomResetCall","Reset all rooms for the currently authenticated player for your application. This method is only accessible to whitelisted tester accounts for your application."],["RoomResetForAllPlayerCall","Deletes rooms where the only room participants are from whitelisted tester accounts for your application. This method is only available to user accounts for your developer console."],["ScoreMethods","A builder providing access to all methods supported on *score* resources. It is not used directly, but through the `GamesManagement` hub."],["ScoreResetAllCall","Resets all scores for all leaderboards for the currently authenticated players. This method is only accessible to whitelisted tester accounts for your application."],["ScoreResetAllForAllPlayerCall","Resets scores for all draft leaderboards for all players. This method is only available to user accounts for your developer console."],["ScoreResetCall","Resets scores for the leaderboard with the given ID for the currently authenticated player. This method is only accessible to whitelisted tester accounts for your application."],["ScoreResetForAllPlayerCall","Resets scores for the leaderboard with the given ID for all players. This method is only available to user accounts for your developer console. Only draft leaderboards can be reset."],["ScoreResetMultipleForAllPlayerCall","Resets scores for the leaderboards with the given IDs for all players. This method is only available to user accounts for your developer console. Only draft leaderboards may be reset."],["ScoresResetMultipleForAllRequest","This is a JSON template for multiple scores reset all request."],["TurnBasedMatcheMethods","A builder providing access to all methods supported on *turnBasedMatche* resources. It is not used directly, but through the `GamesManagement` hub."],["TurnBasedMatcheResetCall","Reset all turn-based match data for a user. This method is only accessible to whitelisted tester accounts for your application."],["TurnBasedMatcheResetForAllPlayerCall","Deletes turn-based matches where the only match participants are from whitelisted tester accounts for your application. This method is only available to user accounts for your developer console."]],"enum":[["Error",""],["Scope","Identifies the an OAuth2 authorization scope. A scope is needed when requesting an authorization token."]],"fn":[["remove_json_null_values",""]],"trait":[["CallBuilder","Identifies types which represent builders for a particular resource method"],["Delegate","A trait specifying functionality to help controlling any request performed by the API. The trait has a conservative default implementation."],["Hub","Identifies the Hub. There is only one per library, this trait is supposed to make intended use more explicit. The hub allows to access all resource methods more easily."],["MethodsBuilder","Identifies types for building methods of a particular resource type"],["NestedType","Identifies types which are only used by other types internally. They have no special meaning, this trait just marks them for completeness."],["Part","Identifies types which are only used as part of other types, which usually are carrying the `Resource` trait."],["ReadSeek","A utility to specify reader types which provide seeking capabilities too"],["RequestValue","Identifies types which are used in API requests."],["Resource","Identifies types which can be inserted and deleted. Types with this trait are most commonly used by clients of this API."],["ResponseResult","Identifies types which are used in API responses."],["ToParts","A trait for all types that can convert themselves into a *parts* string"]]});