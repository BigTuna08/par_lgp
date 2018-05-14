dim1 = 2;
dim2 = 5;

csf = 'cv_fits' %compare_stat_folder
stat = 'max';


d = dir('./*');
for dd = d'
    fname = dd.name;
    n1 = fname(1);
    if n1 == '.'
        continue
    end
    
    parts = strsplit(fname, '_');
    num1 = str2double(parts(dim1))
    num2 = str2double(parts(dim2))
    
end