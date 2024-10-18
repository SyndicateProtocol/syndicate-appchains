pragma solidity ^0.8.0;

contract L3BackfillBetterMapper {
    struct Interval {
        uint32 endDiff; // Difference from previous end
        uint224 value; // Allows for large values while saving storage
    }

    Interval[] public intervals;
    uint256 public lastEnd;

    function addInterval(uint256 end, uint224 value) public {
        require(end > lastEnd, "Intervals must be in order and non-overlapping");
        uint32 endDiff = uint32(end - lastEnd);
        intervals.push(Interval(endDiff, value));
        lastEnd = end;
    }

    function query(uint256 x) public view returns (uint224) {
        require(intervals.length > 0 && x <= lastEnd, "Out of range");

        uint256 left = 0;
        uint256 right = intervals.length - 1;
        uint256 currentEnd = 0;

        while (left <= right) {
            uint256 mid = (left + right) / 2;
            currentEnd += intervals[mid].endDiff;

            if (x <= currentEnd) {
                return intervals[mid].value;
            }
            left = mid + 1;
        }

        revert("No matching interval found");
    }

    function getIntervalsCount() public view returns (uint256) {
        return intervals.length;
    }
}
