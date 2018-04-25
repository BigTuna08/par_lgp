function plot_distr(in_file, column)
%column optional arg, used to select time step
%if not given final column is used (final result)
M = dlmread(in_file);
if exist('column','var')
    data = M(:,column)';
else
    data = M(:, end)';
end

n = length(data);
ave = sum(data)/n
sd = sqrt(sum((data - ave).^2) / n)

histogram(data);
end