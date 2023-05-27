# OSRM Table Service Bulk API

Currently, the table service api of OSRM support only 100 pairs of latitude, and longitude. What if we require the response for more than 100 pairs all at once. Under the hood, this lib will process through chunking algorithm.