loc = 'eval6/test_fit_maps/'

folder = dir(strcat(loc,'*.txt'));

count = 0;

for file = folder
    file.name %gets stored as ans
    if count == 0
        M = dlmread( strcat(loc,ans) );
    else
        M = M + dlmread( strcat(loc,ans) );
    end
    count = count + 1;
end

M/count;