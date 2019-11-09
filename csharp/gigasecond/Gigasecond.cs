using System;

public static class Gigasecond
{
    private const int DELTA = 1000000000;

    public static DateTime Add(DateTime moment)
    {
        return moment.AddSeconds(DELTA);
    }
}