# rechaud

Renamer for shows and anime

Before:
```
Shows
    The Office
        Season 1
            01 - First episode
            02 - Second episode
        Season 2
            01 - First episode
            02 - Second episode
        ...
        Season 11
            The Final Episode
    Big Bang Theory
        Season 1
            FirstEpisode
            FourthEpisode
            SecondEpisode
            ThirdEpisode
            ...
            TenthEpisode
```

After:
```
Shows
    The Office
        S01
            S01E1
            S01E2
        S02
            S02E1
            S02E2
        ..
        S11
            S11E1
    Big Bang Theory
        S1
            S1E01
            S1E02
            S1E03
            S1E04
            ...
            S1E10
```

In order to work even if the files are named incorrectly and thus mixed (for instance first episode not being the first file in the folder), the tool will let you change the episodes / seasons order if needed.